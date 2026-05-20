//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 902/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk902<F: Float>(t3006: F, t974: F, t3014: F, t972: F, t2873: F, t910: F, t2876: F, t11300: F, t935: F, t2924: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F) -> (F, F, F, F, F) {
    let t11521 = t974 * t3006;
    let t11524 = t3006 * t3014;
    let t11525 = t11524 * t972;
    let t11528 = t910 * t2873;
    let t11530 = F::new(6.0) * t11528 * t2876;
    let t11531 = t11300 * t935;
    let t11533 = F::new(6.0) * t2924 * t11531;
    let t11534 = F::cast_from(0.55403703703703703703e-1_f64) * t11132;
    let t11545 = -t11534 - F::cast_from(0.23744444444444444444e-1_f64) * t11134 + F::cast_from(0.11872222222222222222e-1_f64) * t11136 - F::cast_from(0.35616666666666666666e-1_f64) * t11138 + F::cast_from(0.17808333333333333333e-1_f64) * t11140 - F::cast_from(0.19787037037037037037e-1_f64) * t11147 + F::cast_from(0.71233333333333333332e-1_f64) * t11153 - F::cast_from(0.35616666666666666666e-1_f64) * t11158 - F::new(0.10685e0) * t11162 + F::new(0.10685e0) * t11167 - F::cast_from(0.17808333333333333333e-1_f64) * t11171;
    (t11521, t11525, t11530, t11533, t11545)
}
