//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 691/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk691(t1016: f64, t9243: f64, t3366: f64, t6556: f64, t3145: f64, t4349: f64, t3207: f64, t1382: f64, t12766: f64, t1445: f64, t597: f64, t1645: f64, t3137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12851 = t9243 * t1016;
    let t12853 = 4.0_f64 * t6556 * t3366;
    let t12856 = t1016 * t3145;
    let t12858 = 6.0_f64 * t4349 * t12856;
    let t12862 = t1016 * t3207;
    let t12864 = 2.0_f64 * t1382 * t12862;
    let t12868 = t1445 * t12766;
    let t12870 = 0.11502877786176224903e2_f64 * t597 * t12868;
    let t12871 = t1645 * t3137;
    (t12851, t12853, t12856, t12858, t12862, t12864, t12868, t12870, t12871)
}
