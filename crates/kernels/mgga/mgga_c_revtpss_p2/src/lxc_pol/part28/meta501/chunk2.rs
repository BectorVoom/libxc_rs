//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1890/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1890<F: Float>(t25956: F, t26087: F, t532: F, t1450: F, t2014: F, t118: F, t2011: F, t2322: F, t2331: F, t2372: F, t25800: F, t25804: F, t25805: F, t25835: F, t25838: F, t25840: F, t25842: F, t25844: F, t25846: F, t25853: F, t25858: F, t25860: F, t25863: F, t25868: F, t25872: F, t4151: F, t569: F, t651: F, t671: F, t6985: F, t7007: F) -> (F, F, F, F) {
    let t26088 = t25956 + t26087;
    let t26089 = t532 * t26088;
    let t26090 = t26089 * t1450;
    let t26091 = t2014 * t26090;
    let t26092 = -t118 * t25800 + t2011 * t4151 - F::cast_from(4.0_f64) * t2322 * t7007 - F::cast_from(4.0_f64) * t2331 * t6985 - F::cast_from(2.0_f64) * t2372 * t6985 - F::cast_from(4.0_f64) * t25805 * t671 + t25835 * t569 - F::cast_from(4.0_f64) * t25872 * t651 - t25804 + t25838 - t25840 - t25842 - t25844 + t25846 - t25853 - t25858 - t25860 - t25863 + t25868 + t26091;
    (t26088, t26089, t26090, t26092)
}
