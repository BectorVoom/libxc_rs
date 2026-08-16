//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 895/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk895(t1897: f64, t28957: f64, t2936: f64, t3266: f64, t8942: f64, t2508: f64, t32658: f64, t954: f64, t40746: f64, t40750: f64, t40752: f64, t40758: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43131 = 0.23071578690426672851e-1_f64 * t1897 * t2936 * t28957;
    let t43143 = 0.76905262301422242837e-2_f64 * t1897 * t3266 * t8942;
    let t43146 = 0.15381052460284448567e-1_f64 * t2508 * t954 * t32658;
    let t43148 = 0.64087718584518535698e-3_f64 * t40746;
    let t43152 = 0.64087718584518535698e-3_f64 * t40750;
    let t43156 = 0.64087718584518535698e-3_f64 * t40752;
    let t43157 = 0.64087718584518535698e-3_f64 * t40758;
    (t43131, t43143, t43146, t43148, t43152, t43156, t43157)
}
