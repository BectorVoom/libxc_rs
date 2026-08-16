//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2475;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta674<F: Float>(t3494: F, t3519: F, t12258: F, t698: F, t13026: F, t240: F, t3361: F, t1146: F, t9303: F, t12270: F, t2304: F, t2439: F, t3424: F, t3421: F, t12278: F, t12274: F, t25273: F, t268: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43752, t43762, t43764, t43766, t43771, t43773, t43776, t43781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2475::<F>(t3494, t3519, t12258, t698, t13026, t240, t3361, t1146, t9303, t12270, t2304, t2439, t3424);
        let (t43783, t43785, t43787, t43813) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2476::<F>(t2439, t3421, t12278, t698, t12274, t25273, t268, t404);
    (t43752, t43762, t43764, t43766, t43771, t43773, t43776, t43781, t43783, t43785, t43787, t43813)
}
