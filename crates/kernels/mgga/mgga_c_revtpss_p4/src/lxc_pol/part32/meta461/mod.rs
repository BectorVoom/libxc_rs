//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1682;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1683;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta461<F: Float>(t239: F, t25981: F, t820: F, t240: F, t7262: F, t3994: F, t2661: F, t2482: F, t27: F, t4021: F, t25273: F, t533: F, t816: F, t540: F, t7021: F, t1372: F, t1389: F, t7269: F, t2736: F, t2689: F, t7256: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25983, t25986) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1682::<F>(t239, t25981, t820, t240, t7262);
        let (t25987, t25989, t25997) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1683::<F>(t25986, t3994, t2661, t2482, t27, t7262);
        let (t25998, t26002, t26004, t26006, t26009, t26010, t26012) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1684::<F>(t25997, t4021, t25273, t533, t816, t540, t7021, t1372, t1389, t7269, t2736, t2689, t7256);
    (t25983, t25986, t25987, t25989, t25997, t25998, t26002, t26004, t26006, t26009, t26010, t26012)
}
