//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 646/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk646<F: Float>(t4298: F, t4299: F, t1755: F, t707: F, t1759: F, t1763: F, t113: F, t2803: F, t301: F, t1166: F, t413: F, t297: F, t1183: F, t398: F, t122: F, t4182: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4301 = 4.569219094474146e-06 * t4298 * t4299;
    let t4302 = t707 * t1755;
    let t4304 = t707 * t1759;
    let t4307 = 0.05987117005127304 * t707 * t1763;
    let t4309 = t2803 * t113 * t301;
    let t4313 = t1166 * t413 * t301;
    let t4314 = t297 * t4313;
    let t4317 = t398 * t1183 * t301;
    let t4318 = t297 * t4317;
    let t4320 = t122 * t4182;
    (t4301, t4302, t4304, t4307, t4309, t4313, t4314, t4317, t4318, t4320)
}
