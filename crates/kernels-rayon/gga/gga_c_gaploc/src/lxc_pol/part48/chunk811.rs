//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 811/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk811(t32840: f64, t3295: f64, t9805: f64, t11053: f64, t9829: f64, t20671: f64, t28856: f64, t32847: f64, t13058: f64, t28737: f64, t33289: f64, t9800: f64, t9806: f64) -> (f64, f64, f64, f64, f64) {
    let t43373 = t9805 * t32840 * t3295;
    let t43377 = t9805 * t11053 * t9829;
    let t43383 = t28856 * t20671 * t32847;
    let t43386 = t28737 * t13058;
    let t43389 = t9800 * t33289 * t9806;
    (t43373, t43377, t43383, t43386, t43389)
}
