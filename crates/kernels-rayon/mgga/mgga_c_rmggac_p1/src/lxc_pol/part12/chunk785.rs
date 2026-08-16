//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 785/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk785(t1987: f64, t7921: f64, t1990: f64, t1993: f64, t7920: f64, t1997: f64, t7335: f64, t7927: f64, t16156: f64, t7742: f64, t7380: f64, t5542: f64, t7546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36513 = t7921 * t1987;
    let t36515 = t7921 * t1990;
    let t36520 = t1993 * t7920;
    let t36521 = t36520 * t1997;
    let t36527 = t7335 * t7927;
    let t36528 = 0.12195059916630011326e-2_f64 * t36527;
    let t36533 = t16156 * t7742;
    let t36535 = t16156 * t7380;
    let t36541 = t7546 * t5542;
    (t36513, t36515, t36520, t36521, t36528, t36533, t36535, t36541)
}
