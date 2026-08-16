//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 798/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk798<F: Float>(t11053: F, t9805: F, t9829: F, t20671: F, t28856: F, t32847: F, t13058: F, t28737: F, t33289: F, t9800: F, t9806: F, t11068: F, t2679: F, t9796: F) -> (F, F, F, F, F) {
    let t43377 = t9805 * t11053 * t9829;
    let t43383 = t28856 * t20671 * t32847;
    let t43386 = t28737 * t13058;
    let t43389 = t9800 * t33289 * t9806;
    let t43400 = t9796 * t11068 * t2679;
    (t43377, t43383, t43386, t43389, t43400)
}
