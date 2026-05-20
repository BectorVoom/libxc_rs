//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2459/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2459<F: Float>(t11262: F, t3161: F, t3163: F, t11267: F, t3123: F, t11922: F, t12016: F, t3115: F, t11638: F, t3127: F, t3172: F, t11683: F, t11710: F, t3091: F) -> (F, F, F, F, F) {
    let t42932 = t3161 * t11262 * t3163;
    let t42934 = t3123 * t11267;
    let t42947 = t3115 * t11922 * t12016;
    let t42962 = t3127 * t3172 * t11638;
    let t42965 = t3091 * t11710 * t11683;
    (t42932, t42934, t42947, t42962, t42965)
}
