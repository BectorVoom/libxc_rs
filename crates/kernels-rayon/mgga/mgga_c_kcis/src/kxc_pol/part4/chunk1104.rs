//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1104/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1104(t13926: f64, t4714: f64, t13480: f64, t2970: f64, t26: f64, t13495: f64, t945: f64, t13714: f64, t13744: f64, t939: f64, t1676: f64, t2331: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13927 = t4714 * t13926;
    let t13930 = t2970 * t13480;
    let t13931 = t26 * t13930;
    let t13933 = t945 * t13495;
    let t13934 = t26 * t13933;
    let t13939 = 0.39862222222222222222e0_f64 * t13714;
    let t13942 = t939 * t13744;
    let t13945 = t2331 * t1676;
    (t13927, t13931, t13934, t13939, t13942, t13945)
}
