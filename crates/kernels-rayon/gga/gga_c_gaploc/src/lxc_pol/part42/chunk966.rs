//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 966/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk966(t3720: f64, t8469: f64, t14364: f64, t835: f64, t723: f64, t325: f64, t701: f64, t12161: f64, t12177: f64, t12182: f64, t14357: f64, t14384: f64, t14388: f64, t14391: f64, t1445: f64, t1457: f64, t1998: f64, t2004: f64, t2087: f64, t2103: f64, t3009: f64, t3040: f64, t43425: f64, t45229: f64, t45232: f64, t45234: f64, t45238: f64, t45242: f64, t45243: f64, t45247: f64, t4614: f64, t5771: f64, t807: f64, t813: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t50043 = t8469 * t3720;
    let t50050 = t835 * t14364;
    let t50051 = t50050 * t723;
    let t50062 = t325 * t14364;
    let t50063 = t50062 * t701;
    let t50074 = -0.13803453343411469884e2_f64 * t2087 * t1445 * t3009 * t12161 + 0.30674340763136599741e2_f64 * t833 * t4614 * t14384 - 0.12269736305254639897e2_f64 * t813 * t4614 * t14391 + 0.14300195980740170668e1_f64 * t5771 * t14388 + 0.14300195980740170668e1_f64 * t2103 * t1457 * t50043 - t45229 - t45232 + 0.57514388930881124514e0_f64 * t45234 - 0.85206502119823888169e0_f64 * t43425 - 0.51762950037793012063e1_f64 * t45238 + t45242 - t45243 + t45247 + 0.71500979903700853338e0_f64 * t2103 * t1457 * t50051 - 0.18404604457881959845e2_f64 * t2087 * t4614 * t14357 + 0.71500979903700853338e0_f64 * t12182 * t3040 + 0.71500979903700853338e0_f64 * t12177 * t3040 + 0.35750489951850426669e0_f64 * t2004 * t1457 * t50063 + 0.23005755572352449806e1_f64 * t807 * t1445 * t50063 - 0.23005755572352449806e1_f64 * t1998 * t1445 * t50050 * t701;
    (t50043, t50051, t50062, t50063, t50074)
}
