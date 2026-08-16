//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1091/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1091(t4742: f64, t477: f64, t5077: f64, t5094: f64, t1438: f64, t154: f64, t12398: f64, t3098: f64, t5083: f64, t12973: f64, t12974: f64, t12975: f64, t12976: f64, t12977: f64, t12978: f64, t12979: f64, t12983: f64, t12986: f64) -> (f64, f64, f64, f64, f64) {
    let t12987 = t4742 * t477;
    let t12990 = 2.0_f64 / 15.0_f64 * t5077 * t5094 * t12987;
    let t12991 = t154 * t1438;
    let t12994 = 2.0_f64 / 5.0_f64 * t5077 * t12991 * t12398;
    let t12995 = t154 * t3098;
    let t12998 = 2.0_f64 / 3.0_f64 * t5083 * t12995 * t12398;
    let t12999 = t12973 + t12974 + t12975 + t12976 + t12977 + t12978 + t12979 - t12983 + t12986 + t12990 + t12994 - t12998;
    (t12987, t12990, t12994, t12998, t12999)
}
