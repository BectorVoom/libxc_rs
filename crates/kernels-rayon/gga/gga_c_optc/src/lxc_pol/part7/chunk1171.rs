//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1171/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1171(t24248: f64, t24263: f64, t24279: f64, t24294: f64, t779: f64, t799: f64, t2414: f64, t216: f64, t2374: f64, t2417: f64, t23605: f64, t23608: f64, t23612: f64, t23614: f64, t23616: f64, t23653: f64, t23655: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64) -> (f64, f64, f64, f64) {
    let t24299 = 1.0_f64 * t779 * (t24248 + t24263 + t24279 + t24294) * t799;
    let t24300 = t2414 * t2414;
    let t24302 = t216 / t24300;
    let t24303 = t2374 * t2374;
    let t24304 = t2417 * t2417;
    let t24305 = 1.0_f64 / t24304;
    let t24308 = 0.24954977986735470917e5_f64 * t24302 * t24303 * t24305;
    let t24320 = -0.22249999999999999999e0_f64 * t23605 + 0.22249999999999999999e0_f64 * t23670 - 0.18541666666666666666e-1_f64 * t23608 - 0.24722222222222222222e-1_f64 * t23673 - 0.61805555555555555555e-1_f64 * t23676 + 0.2225e0_f64 * t23612 - 0.33375e0_f64 * t23679 + 0.49444444444444444445e-1_f64 * t23614 + 0.74166666666666666668e-1_f64 * t23616 - 0.74166666666666666668e-1_f64 * t23653 + 0.24722222222222222222e-1_f64 * t23655;
    (t24299, t24303, t24308, t24320)
}
