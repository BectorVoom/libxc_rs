//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 943/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk943(t40750: f64, t10789: f64, t1897: f64, t7671: f64, t40752: f64, t40758: f64, t13185: f64, t7129: f64, t13217: f64, t10673: f64, t2508: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43152 = 0.64087718584518535698e-3_f64 * t40750;
    let t43154 = t1897 * t10789 * t7671;
    let t43156 = 0.64087718584518535698e-3_f64 * t40752;
    let t43157 = 0.64087718584518535698e-3_f64 * t40758;
    let t43166 = 0.53833683610995569986e-1_f64 * t7129 * t13185;
    let t43168 = 0.46143157380853345701e-1_f64 * t7129 * t13217;
    let t43170 = t2508 * t954 * t10673;
    (t43152, t43154, t43156, t43157, t43166, t43168, t43170)
}
