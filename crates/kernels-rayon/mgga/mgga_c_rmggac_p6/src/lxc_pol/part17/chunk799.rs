//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 799/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk799(t7244: f64, t9159: f64, t1607: f64, t1986: f64, t7279: f64, t8365: f64, t2283: f64, t7921: f64, t2185: f64, t8675: f64, t1997: f64, t1540: f64, t880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38934 = t7244 * t9159;
    let t38943 = t1986 * t1607;
    let t38946 = t8365 * t7279;
    let t38965 = t7921 * t2283;
    let t38967 = t8675 * t2185;
    let t38968 = t38967 * t1997;
    let t38969 = 0.24829349937757072982e-4_f64 * t38968;
    let t38973 = t1540 * t880;
    (t38934, t38943, t38946, t38965, t38967, t38969, t38973)
}
