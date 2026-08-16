//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1341/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1341(t17276: f64, t1650: f64, t3954: f64, t4163: f64, t4162: f64, t4160: f64, t1924: f64, t3960: f64, t11862: f64, t5638: f64, t1928: f64, t4169: f64) -> (f64, f64, f64, f64, f64) {
    let t17277 = 0.14739506172839506172e-2_f64 * t17276;
    let t17279 = t4163 * t1650 * t3954;
    let t17280 = t4162 * t17279;
    let t17281 = t4160 * t17280;
    let t17287 = t1924 * t3960;
    let t17290 = t11862 * t5638;
    let t17292 = t4169 * t1928;
    (t17277, t17281, t17287, t17290, t17292)
}
