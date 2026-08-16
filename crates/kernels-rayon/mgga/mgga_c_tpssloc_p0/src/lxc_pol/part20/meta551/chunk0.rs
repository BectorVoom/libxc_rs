//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2096/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2096(t2250: f64, t4194: f64, t607: f64, t750: f64, t2617: f64, t9670: f64, t831: f64, t236: f64, t40931: f64, t2638: f64, t9612: f64, t10021: f64, t812: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41295 = t4194 * t750 * t607 * t2250;
    let t41340 = t2617 * t9670;
    let t41341 = t41340 * t831;
    let t41347 = t40931 * t236;
    let t41354 = t9612 * t2638;
    let t41355 = t41354 * t831;
    let t41362 = t812 * t815 * t10021;
    (t41295, t41340, t41341, t41347, t41354, t41355, t41362)
}
