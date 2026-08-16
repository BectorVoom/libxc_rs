//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2591/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2591(t15438: f64, t19095: f64, t19083: f64, t4993: f64, t18392: f64, t5024: f64, t1226: f64, t22115: f64, t11692: f64, t1174: f64, t1177: f64, t1232: f64, t15700: f64, t15740: f64, t1735: f64, t18221: f64, t18397: f64, t18401: f64, t19010: f64, t19106: f64, t3440: f64, t3577: f64, t3578: f64, t4889: f64, t52766: f64, t53298: f64, t5392: f64, t65528: f64, t71172: f64, t71193: f64) -> f64 {
    let t72248 = t15438 * t19095;
    let t72251 = t19083 * t4993;
    let t72253 = t5024 * t18392;
    let t72255 = t22115 * t1226;
    let t72268 = t52766 * t18397 / 768.0_f64 - t15740 * t18401 / 384.0_f64 + t11692 * t3578 * t15700 * t53298 * t5392 / 768.0_f64 - t3577 * t3578 * t1735 * t18221 / 256.0_f64 - t72248 / 1536.0_f64 - t65528 / 4608.0_f64 + t72251 / 216.0_f64 + t72253 / 216.0_f64 - t72255 * t1232 / 4608.0_f64 + t4889 * t19010 / 18.0_f64 - t1174 * t1177 * t71172 / 12.0_f64 + t1174 * t3440 * t71193 / 12.0_f64 + 7.0_f64 / 81.0_f64 * t4889 * t19106;
    t72268
}
