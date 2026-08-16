//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 895/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk895(t11801: f64, t41105: f64, t1445: f64, t2639: f64, t43425: f64, t43435: f64, t44967: f64, t45066: f64, t45212: f64, t45215: f64, t45217: f64, t45219: f64, t45222: f64, t45226: f64, t45229: f64, t45232: f64, t45234: f64, t45238: f64, t45242: f64, t45243: f64, t45247: f64, t45251: f64, t45252: f64, t833: f64) -> f64 {
    let t45256 = 0.42900587942220512003e1_f64 * t11801 * t41105;
    let t45257 = 0.23005755572352449806e2_f64 * t833 * t1445 * t44967 + 0.23005755572352449806e2_f64 * t833 * t1445 * t45066 + t45212 + t45215 + t45217 - 0.76685851907841499354e0_f64 * t45219 - 0.76685851907841499354e0_f64 * t45222 + t45226 - t45229 - t45232 + 0.57514388930881124515e0_f64 * t45234 - 0.85206502119823888171e0_f64 * t43425 - 0.51762950037793012064e1_f64 * t45238 + t45242 - t45243 - 0.15337170381568299871e1_f64 * t43435 + t45247 - t45251 - 0.10725146985555128001e1_f64 * t45252 * t2639 + t45256;
    t45257
}
