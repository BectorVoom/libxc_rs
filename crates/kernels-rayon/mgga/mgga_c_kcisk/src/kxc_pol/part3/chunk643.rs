//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 643/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk643(t1596: f64, t4350: f64, t2028: f64, t5439: f64, t3182: f64, t1065: f64, t3462: f64, t1156: f64, t4569: f64, t294: f64, t1008: f64, t195: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9517 = t4350 * t1596;
    let t9726 = t5439 * t2028;
    let t10328 = 6.0_f64 * t3182;
    let t10329 = t1065 * t3462;
    let t10330 = 3.0_f64 * t10329;
    let t10331 = t1156 * t4569;
    let t10332 = t294 * t10331;
    let t10333 = 3.0_f64 / 16.0_f64 * t10332;
    let t10334 = t1008 * t1008;
    let t10335 = 1.0_f64 / t10334;
    let t10336 = t195 * t10335;
    (t9517, t9726, t10328, t10330, t10333, t10336)
}
