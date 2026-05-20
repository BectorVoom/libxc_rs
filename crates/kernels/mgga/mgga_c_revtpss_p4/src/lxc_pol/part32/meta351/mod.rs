//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1287;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta351<F: Float>(t14370: F, t4401: F, t4391: F, t705: F, t2615: F, t4311: F, t1469: F, t2609: F, t706: F, t1568: F, t785: F, t780: F, t2439: F, t212: F, t4469: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14372, t14386, t14433, t14441, t14473) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1287::<F>(t14370, t4401, t4391, t705, t2615, t4311, t1469, t2609, t706, t1568, t785, t780);
        let (t14474, t14479, t14481, t14484, t14485) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1288::<F>(t14473, t2439, t212, t4469, t780, t689, t1579, t2769, t886, t252, t2782, t2470, t4480);
    (t14372, t14386, t14433, t14441, t14474, t14479, t14481, t14484, t14485)
}
