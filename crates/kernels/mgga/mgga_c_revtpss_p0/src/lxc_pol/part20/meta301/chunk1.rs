//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1186/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1186<F: Float>(t12378: F, t448: F, t300: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F) {
    let t12379 = t12378 * t448;
    let t12381 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t12379;
    let t12382 = F::cast_from(0.55403703703703703703e-1_f64) * t12295;
    let t12393 = -t12382 + F::cast_from(0.23744444444444444444e-1_f64) * t12297 + F::cast_from(0.11872222222222222222e-1_f64) * t12299 - F::cast_from(0.35616666666666666666e-1_f64) * t12301 - F::cast_from(0.17808333333333333333e-1_f64) * t12303 + F::cast_from(0.19787037037037037037e-1_f64) * t12307 - F::cast_from(0.71233333333333333332e-1_f64) * t12310 - F::cast_from(0.35616666666666666666e-1_f64) * t12292 + F::new(0.10685e0) * t12314 + F::new(0.10685e0) * t12317 + F::cast_from(0.17808333333333333333e-1_f64) * t12320;
    (t12379, t12381, t12393)
}
