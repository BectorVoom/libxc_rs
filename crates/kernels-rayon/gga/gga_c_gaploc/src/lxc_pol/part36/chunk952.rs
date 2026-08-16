//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 952/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk952(t40890: f64, t2508: f64, t43007: f64, t688: f64, t779: f64, t1897: f64, t28720: f64, t9014: f64, t40896: f64, t28024: f64, t2936: f64, t13172: f64, t1901: f64, t43243: f64, t43244: f64, t43248: f64, t43254: f64, t43257: f64, t43260: f64, t43263: f64, t43265: f64, t43267: f64, t43269: f64, t43270: f64, t43274: f64) -> f64 {
    let t43275 = 0.2563508743380741428e-2_f64 * t40890;
    let t43278 = t2508 * t779 * t43007 * t688;
    let t43282 = 0.92286314761706691403e-1_f64 * t1897 * t9014 * t28720;
    let t43283 = 0.17090058289204942853e-2_f64 * t40896;
    let t43286 = 0.53833683610995569986e-1_f64 * t2508 * t2936 * t28024;
    let t43287 = -t43243 - 0.46143157380853345702e-1_f64 * t43244 - 0.46143157380853345702e-1_f64 * t43248 + 0.76905262301422242837e-2_f64 * t2508 * t779 * t13172 + t43254 + t43257 + t43260 + t43263 + t43265 - t43267 - t43269 - 0.23071578690426672851e-1_f64 * t2508 * t1901 * t43270 - t43274 + t43275 + 0.15381052460284448567e-1_f64 * t43278 - t43282 - t43283 - t43286;
    t43287
}
