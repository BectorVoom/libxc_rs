//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 974/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk974(t2890: f64, t9959: f64, t991: f64, t2877: f64, t984: f64, t2810: f64, t296: f64, t3132: f64, t738: f64, t3136: f64, t743: f64, t3139: f64, t733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9960 = t9959 * t2890;
    let t9961 = t991 * t9960;
    let t9970 = t984 * t2877;
    let t9985 = 1.0_f64 / t2810 / t296;
    let t10021 = t738 * t3132;
    let t10026 = t743 * t3136;
    let t10028 = t733 * t3139;
    (t9961, t9970, t9985, t10021, t10026, t10028)
}
