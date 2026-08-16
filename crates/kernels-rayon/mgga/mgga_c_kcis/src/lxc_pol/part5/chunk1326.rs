//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1326/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1326(t21922: f64, t4160: f64, t17292: f64, t5649: f64, t5655: f64, t20974: f64, t5662: f64, t4162: f64, t5661: f64, t11854: f64, t20979: f64, t4170: f64) -> (f64, f64, f64, f64, f64) {
    let t21923 = t4160 * t21922;
    let t21925 = t17292 * t5649;
    let t21926 = t4160 * t21925;
    let t21928 = t17292 * t5655;
    let t21929 = t4160 * t21928;
    let t21931 = t5662 * t20974;
    let t21932 = t4162 * t21931;
    let t21933 = t5661 * t21932;
    let t21935 = t11854 * t20979;
    let t21936 = t4170 * t21935;
    (t21923, t21926, t21929, t21933, t21936)
}
