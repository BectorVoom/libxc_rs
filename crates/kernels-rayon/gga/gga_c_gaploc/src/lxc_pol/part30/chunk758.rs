//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 758/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk758(t2555: f64, t7187: f64, t2549: f64, t2559: f64, t481: f64, t685: f64, t729: f64, t1897: f64, t2508: f64, t2542: f64, t5269: f64, t7129: f64, t7137: f64, t7141: f64, t7144: f64, t7147: f64, t7152: f64, t7158: f64, t7161: f64, t7165: f64, t7168: f64, t7170: f64, t7175: f64, t7179: f64, t7182: f64, t7184: f64) -> (f64, f64, f64, f64) {
    let t7188 = t7187 * t2555;
    let t7190 = t2549 * t2559;
    let t7193 = t481 * t729 * t685;
    let t7194 = t7193 * t2555;
    let t7196 = 0.99692006687028833308e-3_f64 * t7141 - 0.46143157380853345702e-1_f64 * t2508 * t7144 + 0.76905262301422242837e-2_f64 * t2508 * t7147 - 0.46143157380853345702e-1_f64 * t7129 * t2542 + 0.15381052460284448567e-1_f64 * t5269 * t7152 - 0.61524209841137794271e-1_f64 * t7137 * t2542 + 0.92286314761706691403e-1_f64 * t2508 * t7158 - 0.53833683610995569986e-1_f64 * t2508 * t7161 - 0.15381052460284448567e-1_f64 * t1897 * t7165 + 0.85450291446024714264e-3_f64 * t7168 + 0.46143157380853345702e-1_f64 * t1897 * t7170 - 0.32043859292259267849e-3_f64 * t7175 - 0.32043859292259267849e-3_f64 * t7179 + 0.32043859292259267849e-3_f64 * t7182 - 0.85450291446024714264e-3_f64 * t7184 - 0.64087718584518535698e-3_f64 * t7188 + 0.64087718584518535698e-3_f64 * t7190 + 0.85450291446024714264e-3_f64 * t7194;
    (t7188, t7190, t7194, t7196)
}
