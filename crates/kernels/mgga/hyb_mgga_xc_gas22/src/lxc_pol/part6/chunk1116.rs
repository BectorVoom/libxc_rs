//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1116/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1116<F: Float>(t260: F, t4305: F, t10888: F, t10890: F, t10893: F, t10898: F, t10913: F, t10915: F, t10922: F, t10924: F, t6969: F, t7116: F, t9008: F, t9189: F) -> (F, F) {
    let t10979 = t260 * t4305;
    let t10992 = F::cast_from(0.19419375e1_f64) * t10888 - F::cast_from(0.258925e1_f64) * t10890 - F::cast_from(0.1294625e1_f64) * t10893 + F::cast_from(0.258925e1_f64) * t10915 - t7116 + F::cast_from(0.40256666666666666667e0_f64) * t6969 + F::cast_from(0.80513333333333333333e0_f64) * t9008 - t9189 - F::cast_from(0.301925e0_f64) * t10898 + F::cast_from(0.905775e0_f64) * t10913 - F::cast_from(0.412621875e-1_f64) * t10922 + F::cast_from(0.16504875e0_f64) * t10924;
    (t10979, t10992)
}
