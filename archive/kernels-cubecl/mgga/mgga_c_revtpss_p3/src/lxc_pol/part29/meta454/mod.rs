//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1696;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1697;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta454<F: Float>(t26009: F, t2736: F, t2689: F, t7256: F, t2018: F, t3951: F, t807: F, t1941: F, t550: F, t3946: F, t1389: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F, t1401: F, t241: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26010, t26012, t26014, t26015, t26016, t26018, t26021) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1696::<F>(t26009, t2736, t2689, t7256, t2018, t3951, t807, t1941, t550, t3946, t1389, t25240, t3964);
        let t26024 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1697::<F>(t7262, t820, t843);
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1698::<F>(t1401, t26024, t241, t7262, t820);
    (t26010, t26012, t26014, t26015, t26016, t26018, t26021, t26024, t26025, t26028)
}
