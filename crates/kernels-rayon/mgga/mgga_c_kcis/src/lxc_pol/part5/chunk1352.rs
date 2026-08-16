//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1352/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1352(t22298: f64, t589: f64, t1505: f64, t7267: f64, t1555: f64, t17308: f64, t2069: f64, t17311: f64, t5900: f64, t5897: f64, t6048: f64, t12338: f64, t7271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22299 = t22298 * t589;
    let t22300 = t7267 * t1505;
    let t22301 = t22300 * t1555;
    let t22303 = 2.0_f64 * t17308 * t2069;
    let t22305 = 4.0_f64 * t17311 * t5900;
    let t22307 = 2.0_f64 * t5897 * t6048;
    let t22309 = 2.0_f64 * t12338 * t7271;
    (t22299, t22301, t22303, t22305, t22307, t22309)
}
