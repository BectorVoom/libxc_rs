//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1067/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1067(t1820: f64, t7766: f64, t3330: f64, t2189: f64, t5189: f64, t3325: f64, t8081: f64, t1203: f64, t1176: f64, t1796: f64, t377: f64, t5164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28002 = t7766 * t1820;
    let t28004 = 2.0_f64 * t3330 * t28002;
    let t28005 = t2189 * t5189;
    let t28007 = 2.0_f64 * t3330 * t28005;
    let t28008 = t3325 * t8081;
    let t28009 = t8081 * t1203;
    let t28011 = 2.0_f64 * t3330 * t28009;
    let t28012 = t1796 * t1176;
    let t28014 = t5164 * t377;
    (t28002, t28004, t28005, t28007, t28008, t28009, t28011, t28012, t28014)
}
