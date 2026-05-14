//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 970/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk970<F: Float>(t2007: F, t3177: F, t1511: F, t1980: F, t2012: F, t1420: F, t5171: F, t439: F, t805: F, t9373: F, t2002: F, t3255: F, t13144: F, t13149: F, t13151: F, t13153: F, t13156: F, t13158: F, t13160: F) -> (F, F, F, F, F, F) {
    let t13162 = t3177 * t2007 / 15.0;
    let t13165 = 2.0 / 15.0 * t1511 * t1980 * t2012;
    let t13167 = t1420 * t5171 / 15.0;
    let t13170 = t439 * t9373 * t805 / 45.0;
    let t13172 = t2002 * t3255 / 45.0;
    let t13173 = -t13144 + t13149 + t13151 + t13153 + t13156 + t13158 + t13160 + t13162 + t13165 + t13167 + t13170 + t13172;
    (t13162, t13165, t13167, t13170, t13172, t13173)
}
