//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1275/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1275(t1170: f64, t2121: f64, t34295: f64, t118142: f64, t118157: f64, t118162: f64, t1201: f64, t1244: f64, t1246: f64, t1729: f64, t2144: f64, t24589: f64, t24788: f64, t27406: f64, t27516: f64, t27550: f64, t32458: f64, t32462: f64, t3247: f64, t32477: f64, t34284: f64, t34303: f64, t3961: f64, t4964: f64, t5011: f64, t8882: f64, t8895: f64) -> f64 {
    let t125378 = t2121 * t1170 * t34295;
    let t125383 = 0.43864908449286038307e-1_f64 * t27406 * t32462 - t118142 + t1244 * t8882 * t5011 * t1246 + 0.54831135561607547883e-2_f64 * t118157 + 0.54831135561607547883e-2_f64 * t24589 * t24788 * t34284 - 0.10966227112321509577e-1_f64 * t24589 * t27550 * t2144 * t3247 * t3961 - 0.54831135561607547883e-2_f64 * t118162 + t4964 * t8895 + t1729 * t32477 + t1201 * t34303 + 0.54831135561607547883e-2_f64 * t125378 + 0.54831135561607547883e-2_f64 * t24589 * t27516 * t32458;
    t125383
}
