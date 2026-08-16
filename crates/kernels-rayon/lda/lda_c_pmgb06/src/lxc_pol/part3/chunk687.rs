//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 687/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk687(t1183: f64, t301: f64, t398: f64, t297: f64, t122: f64, t4182: f64, t302: f64, t1773: f64, t715: f64, t711: f64, t4296: f64, t4301: f64, t4302: f64, t4304: f64, t4307: f64, t4309: f64, t4314: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4317 = t398 * t1183 * t301;
    let t4318 = t297 * t4317;
    let t4320 = t122 * t4182;
    let t4322 = 0.19513566535229734_f64 * t4320 * t302;
    let t4324 = 0.15965645347006147_f64 * t1773 * t715;
    let t4325 = t1773 * t711;
    let t4327 = -t4296 - t4301 + 0.05987117005127304_f64 * t4302 + 0.11974234010254609_f64 * t4304 + t4307 - 0.01197423401025461_f64 * t297 * t4309 - 0.03592270203076383_f64 * t4314 - 0.03592270203076383_f64 * t4318 + t4322 - t4324 - 0.15965645347006147_f64 * t4325;
    (t4317, t4318, t4320, t4322, t4324, t4325, t4327)
}
