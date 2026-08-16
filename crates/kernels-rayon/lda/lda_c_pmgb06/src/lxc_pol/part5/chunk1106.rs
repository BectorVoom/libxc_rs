//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1106/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1106(t16563: f64, t1907: f64, t5077: f64, t13308: f64, t19623: f64, t19627: f64, t5084: f64, t13000: f64, t5083: f64, t13043: f64, t19631: f64, t5094: f64) -> (f64, f64, f64, f64, f64) {
    let t20293 = 2.0_f64 / 15.0_f64 * t5077 * t16563 * t1907;
    let t20296 = 4.0_f64 / 15.0_f64 * t5077 * t13308 * t19623;
    let t20299 = 2.0_f64 / 5.0_f64 * t5077 * t5084 * t19627;
    let t20302 = 2.0_f64 / 3.0_f64 * t5083 * t13000 * t19627;
    let t20305 = 8.0_f64 / 15.0_f64 * t13043 * t5094 * t19631;
    (t20293, t20296, t20299, t20302, t20305)
}
