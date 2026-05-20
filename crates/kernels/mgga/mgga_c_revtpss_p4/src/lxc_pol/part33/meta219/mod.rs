//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1005;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1006;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta219<F: Float>(t5826: F, t70: F, t1470: F, t1486: F, t2275: F, t5819: F, t48: F, t5825: F, t476: F, t53: F, t2282: F, t60: F, sigma2: F, t1480: F, t1483: F, t2290: F, t44: F, t56: F, t61: F, t38: F, t2299: F, t633: F, t2306: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t5820: F, t71: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5827, t5830, t5835, t5838, t5842, t5843, t5848, t5851) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1005::<F>(t5826, t70, t1470, t1486, t2275, t5819, t48, t5825, t476, t53, t2282, t60, sigma2);
        let (t5854, t5855) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1006::<F>(t1480, t1483, t2290, t44, t56, t5835, t5838, t5843, t5848, t5851, t61, t38);
        let (t5868, t5869, t5872) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1007::<F>(t2299, t5819, t5825, t633, t2306, t637, t77, t1471, t1487, t1494, t5820, t5827, t5830, t5855, t71, t85);
    (t5827, t5830, t5835, t5838, t5842, t5843, t5854, t5855, t5868, t5869, t5872)
}
