//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 880/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk880(t17612: f64, t17673: f64, t184: f64, t21: f64, t15625: f64, t17524: f64, t17532: f64, t17535: f64, t17539: f64, t17542: f64, t17545: f64, t185: f64, t3597: f64, t3601: f64, t363: f64, t3674: f64, t3678: f64, t4431: f64, t4845: f64, t5: f64, t620: f64, t623: f64, t920: f64) -> f64 {
    let t17674 = t17612 + t17673;
    let t17675 = t17674 * t184;
    let t17676 = t17675 * t21;
    let t17679 = t3601 * t3674 / 2.0_f64 + t3601 * t3678 + t5 * t3597 * t920 / 2.0_f64 + t5 * t185 * t15625 / 4.0_f64 + t5 * t620 * t4431 / 4.0_f64 + t5 * t17524 * t21 / 4.0_f64 + t5 * t4845 * t363 / 4.0_f64 + t623 * t17532 / 2.0_f64 + t623 * t17535 / 4.0_f64 + t623 * t17539 / 2.0_f64 + t623 * t17542 + t623 * t17545 / 4.0_f64 + t623 * t17676 / 4.0_f64;
    t17679
}
