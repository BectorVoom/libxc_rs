//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1212/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1212(t3129: f64, t61735: f64, t3038: f64, t1041: f64, t10868: f64, t248: f64, t5685: f64, t18086: f64, t3069: f64, t10482: f64, t5872: f64, t5681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61736 = t61735 * t3129;
    let t61739 = t61735 * t3038;
    let t61782 = t1041 * t248 * t10868 * t5685;
    let t61950 = t18086 * t3069;
    let t62079 = t5872 * t10482;
    let t62137 = t1041 * t248 * t10868 * t5681;
    (t61736, t61739, t61782, t61950, t62079, t62137)
}
