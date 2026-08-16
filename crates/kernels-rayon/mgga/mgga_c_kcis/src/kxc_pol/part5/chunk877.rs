//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 877/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk877(t7049: f64, t7266: f64, t589: f64, t2069: f64, t5897: f64, t4189: f64, t4301: f64, t6922: f64, t583: f64, t578: f64, t2035: f64, t2043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7267 = t7049 + t7266;
    let t7268 = t7267 * t589;
    let t7270 = 2.0_f64 * t5897 * t2069;
    let t7271 = t2069 * t2069;
    let t7273 = 2.0_f64 * t4189 * t7271;
    let t7274 = t4301 * t6922;
    let t7275 = t583 * t7274;
    let t7276 = t578 * t7275;
    let t7278 = t2035 * t2043;
    (t7267, t7268, t7270, t7271, t7273, t7275, t7276, t7278)
}
