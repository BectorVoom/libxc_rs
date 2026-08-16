//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1064/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1064(t2167: f64, t6892: f64, t2037: f64, t7122: f64, t7130: f64, t127: f64, t2024: f64, t2113: f64, t2124: f64, t2126: f64, t2168: f64, t22173: f64, t22203: f64, t22788: f64, t22792: f64, t22859: f64, t22872: f64, t22875: f64, t23081: f64, t23083: f64, t23085: f64, t23098: f64, t23105: f64, t673: f64, t675: f64, t696: f64, t6993: f64) -> f64 {
    let t23109 = t2167 * t6892;
    let t23110 = t23109 * t2037;
    let t23117 = t7122 * t7130;
    let t23123 = -0.8463958349005185144e1_f64 * t23081 - 0.14604511302845113195e2_f64 * t23083 - 0.26079484469366273564e0_f64 * t673 * t675 * t23085 * t127 + 0.52158968938732547127e0_f64 * t2113 * t675 * t23085 * t2024 + 0.90685268025055555115e0_f64 * t23098 * t696 * t22788 - 0.10882232163006666614e1_f64 * t6993 * t696 * t22792 + 0.81136173904695073308e0_f64 * t23105 - 0.18137053605011111023e1_f64 * t2168 * t22173 + 0.19184972257745086326e2_f64 * t23110 + 0.10431793787746509425e1_f64 * t2124 * t2126 * t22875 + 0.24182738140014814697e0_f64 * t2168 * t22872 + 0.14604511302845113196e2_f64 * t23117 - 0.60456845350037036744e-1_f64 * t2168 * t22203 - 0.90685268025055555116e-1_f64 * t2168 * t22859;
    t23123
}
