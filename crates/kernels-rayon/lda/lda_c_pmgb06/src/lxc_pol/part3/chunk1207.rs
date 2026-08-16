//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1207/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1207(t12920: f64, t12968: f64, t12969: f64, t12970: f64, t12973: f64, t12974: f64, t12975: f64, t12976: f64, t12977: f64, t12978: f64, t12979: f64, t12983: f64, t12986: f64, t12990: f64, t12994: f64, t12998: f64, t13005: f64, t13009: f64, t13012: f64, t13015: f64, t13018: f64, t13024: f64, t13030: f64) -> (f64, f64) {
    let t14402 = t12920 - t12968 - t12969 - t12970 + t12973 + t12974 + t12975 + t12976 + t12977 + t12978 + t12979;
    let t14403 = -t12983 + t12986 + t12990 + t12994 - t12998 + t13005 + t13009 + t13012 - t13015 + t13018 - t13024 + t13030;
    (t14402, t14403)
}
