//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1340/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1340(t2820: f64, t5659: f64, t86: f64, t5664: f64, t1650: f64, t3722: f64, t4171: f64, t4170: f64, t4160: f64, t11913: f64, t5656: f64, t5638: f64) -> (f64, f64, f64, f64, f64) {
    let t17266 = t86 * t2820 * t5659;
    let t17267 = t17266 * t5664;
    let t17268 = 0.3684876543209876543e-2_f64 * t17267;
    let t17270 = t4171 * t1650 * t3722;
    let t17271 = t4170 * t17270;
    let t17272 = t4160 * t17271;
    let t17274 = t11913 * t5656;
    let t17276 = t11913 * t5638;
    (t17267, t17268, t17272, t17274, t17276)
}
