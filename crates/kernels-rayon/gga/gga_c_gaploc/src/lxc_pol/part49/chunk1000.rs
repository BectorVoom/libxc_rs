//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1000/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1000(t40758: f64, t13185: f64, t7129: f64, t13217: f64, t10673: f64, t2508: f64, t954: f64, t13191: f64, t7137: f64, t33285: f64, t7659: f64, t3276: f64, t8682: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43157 = 0.64087718584518535698e-3_f64 * t40758;
    let t43166 = 0.53833683610995569986e-1_f64 * t7129 * t13185;
    let t43168 = 0.46143157380853345701e-1_f64 * t7129 * t13217;
    let t43170 = t2508 * t954 * t10673;
    let t43173 = 0.12304841968227558854e0_f64 * t7137 * t13191;
    let t43175 = t2508 * t33285 * t7659;
    let t43179 = 0.11535789345213336425e0_f64 * t2508 * t3276 * t8682;
    (t43157, t43166, t43168, t43170, t43173, t43175, t43179)
}
