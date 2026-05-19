//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 687/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk687<F: Float>(t1183: F, t301: F, t398: F, t297: F, t122: F, t4182: F, t302: F, t1773: F, t715: F, t711: F, t4296: F, t4301: F, t4302: F, t4304: F, t4307: F, t4309: F, t4314: F) -> (F, F, F, F, F, F, F) {
    let t4317 = t398 * t1183 * t301;
    let t4318 = t297 * t4317;
    let t4320 = t122 * t4182;
    let t4322 = F::cast_from(0.19513566535229734_f64) * t4320 * t302;
    let t4324 = F::cast_from(0.15965645347006147_f64) * t1773 * t715;
    let t4325 = t1773 * t711;
    let t4327 = -t4296 - t4301 + F::cast_from(0.05987117005127304_f64) * t4302 + F::cast_from(0.11974234010254609_f64) * t4304 + t4307 - F::cast_from(0.01197423401025461_f64) * t297 * t4309 - F::cast_from(0.03592270203076383_f64) * t4314 - F::cast_from(0.03592270203076383_f64) * t4318 + t4322 - t4324 - F::cast_from(0.15965645347006147_f64) * t4325;
    (t4317, t4318, t4320, t4322, t4324, t4325, t4327)
}
