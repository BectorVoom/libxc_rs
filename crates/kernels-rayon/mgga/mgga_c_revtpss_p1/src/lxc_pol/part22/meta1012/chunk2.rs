//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3478/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3478(t1041: f64, t1042: f64, t1045: f64, t1063: f64, t11268: f64, t16208: f64, t19668: f64, t19675: f64, t247: f64, t2862: f64, t3127: f64, t3182: f64, t3188: f64, t373: f64, t42943: f64, t4806: f64, t6302: f64, t6312: f64, t63455: f64, t65357: f64, t65359: f64, t65365: f64, t65370: f64, t65376: f64, t65425: f64, t65431: f64, t65433: f64) -> f64 {
    let t65438 = -0.31758531939310916276e-4_f64 * t65357 - 0.10162730220579493208e-2_f64 * t65359 + 0.47637797908966374414e-3_f64 * t1063 * t247 * t3182 * t63455 - 0.23818898954483187207e-3_f64 * t3127 * t1042 * t4806 * t65365 + 0.63517063878621832552e-3_f64 * t1063 * t1042 * t16208 * t65370 - 0.19055119163586549765e-3_f64 * t65376 - 0.14291339372689912324e-3_f64 * t3127 * t1042 * t19675 * t2862 + 0.95275595817932748828e-3_f64 * t3188 * t19668 - 0.72409452821628889107e-2_f64 * t42943 * t6312 + 0.72409452821628889107e-2_f64 * t11268 * t6302 + 0.21437009059034868486e-3_f64 * t1041 * t1042 * t373 * t65425 * t1045 - 0.15244095330869239812e-2_f64 * t65431 + 0.47637797908966374414e-3_f64 * t1063 * t1042 * t4806 * t65433;
    t65438
}
