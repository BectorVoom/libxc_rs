//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2618/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2618<F: Float>(t18332: F, t4889: F, t11668: F, t11734: F, t1202: F, t1216: F, t15503: F, t15740: F, t1735: F, t18211: F, t18383: F, t18387: F, t18948: F, t21762: F, t22174: F, t22275: F, t3577: F, t488: F, t52615: F, t6192: F, t66500: F, t66512: F, t66515: F, t66518: F) -> F {
    let t73076 = t4889 * t18332;
    let t73078 = -t15740 * t18387 / F::cast_from(768.0_f64) - t15740 * t18383 / F::cast_from(1536.0_f64) - t15503 * t18948 / F::cast_from(48.0_f64) + t52615 * t6192 / F::cast_from(144.0_f64) - F::cast_from(11.0_f64) / F::cast_from(162.0_f64) * t66500 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3577 * t11668 * t1735 * t18211 - F::cast_from(209.0_f64) / F::cast_from(2592.0_f64) * t1202 * t22174 * t488 - t11734 * t22275 / F::cast_from(1024.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3577 * t11668 * t21762 * t1216 - t66512 / F::cast_from(768.0_f64) - t66515 / F::cast_from(256.0_f64) + t66518 / F::cast_from(1536.0_f64) - t73076 / F::cast_from(81.0_f64);
    t73078
}
