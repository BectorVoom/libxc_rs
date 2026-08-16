//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1226/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1226(t1338: f64, t5318: f64, t1352: f64, t12259: f64, t1825: f64, t3866: f64, t5310: f64, t1307: f64, t5187: f64, t3870: f64, t820: f64, t1799: f64, t3719: f64) -> (f64, f64, f64, f64, f64) {
    let t16132 = t1338 * t5318;
    let t16133 = t16132 * t1352;
    let t16136 = t12259 * t1825;
    let t16147 = 35.0_f64 / 576.0_f64 * t3866 * t5310;
    let t16148 = t5187 * t1307;
    let t16150 = t3870 * t820 * t16148;
    let t16153 = t1799 * t3719;
    (t16133, t16136, t16147, t16150, t16153)
}
