//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1025/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1025<F: Float>(t31239: F, t729: F, t762: F, t1424: F, t5064: F, t2568: F, t242: F, t30934: F, t30936: F, t30931: F, t30948: F, t241: F, t258: F, t31097: F, t5073: F, t6154: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31241 = t729 * t762 * t31239;
    let t31244 = t1424 * t5064;
    let t31246 = t729 * t2568 * t31244;
    let t31249 = t242 * t30934;
    let t31252 = t242 * t30936;
    let t31255 = t242 * t30931;
    let t31258 = t242 * t30948;
    let t31262 = t241 * t31097 * t258;
    let t31268 = t729 * t6154 * t5073;
    (t31241, t31244, t31246, t31249, t31252, t31255, t31258, t31262, t31268)
}
