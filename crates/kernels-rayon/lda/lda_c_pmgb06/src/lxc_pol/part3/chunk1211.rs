//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1211/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1211(t13214: f64, t13216: f64, t13219: f64, t13221: f64, t13223: f64, t13225: f64, t13226: f64, t13227: f64, t13231: f64, t13233: f64, t13236: f64, t13238: f64, t13240: f64, t13242: f64, t13244: f64, t13246: f64, t13248: f64, t13250: f64, t13252: f64, t13257: f64, t13258: f64, t13260: f64, t13262: f64) -> (f64, f64) {
    let t14417 = t13214 + t13216 - t13219 + t13221 - t13223 - t13225 - t13226 + t13227 + t13231 + t13233 + t13236;
    let t14418 = t13238 + t13240 + t13242 - t13244 - t13246 - t13248 - t13250 - t13252 - t13257 - t13258 + t13260 + t13262;
    (t14417, t14418)
}
