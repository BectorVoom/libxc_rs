//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1268/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1268(t18392: f64, t5005: f64, t15737: f64, t18356: f64, t19040: f64, t5024: f64, t11738: f64, t22299: f64, t248: f64, t3570: f64, t11728: f64, t22312: f64) -> (f64, f64, f64, f64, f64) {
    let t72285 = t5005 * t18392;
    let t72287 = t15737 * t18356;
    let t72289 = t5024 * t19040;
    let t72293 = t11738 * t248 * t3570 * t22299;
    let t72297 = t11728 * t248 * t3570 * t22312;
    (t72285, t72287, t72289, t72293, t72297)
}
