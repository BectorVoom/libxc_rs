//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1799;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta495<F: Float>(t25997: F, t4021: F, t25273: F, t533: F, t816: F, t540: F, t7021: F, t1372: F, t1389: F, t7269: F, t2736: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F, t1941: F, t550: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25998, t26003, t26004, t26005, t26009, t26011, t26012) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1799::<F>(t25997, t4021, t25273, t533, t816, t540, t7021, t1372, t1389, t7269, t2736, t2689, t7256);
        let (t26013, t26014, t26015, t26017, t26022, t26024) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1800::<F>(t26012, t2018, t3951, t807, t1941, t550, t1389, t25240, t3964, t7262, t820, t843);
    (t25998, t26003, t26004, t26005, t26009, t26011, t26013, t26014, t26015, t26017, t26022, t26024)
}
