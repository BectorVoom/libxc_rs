//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 218/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk218<F: Float>(t903: F, t908: F, t291: F, t287: F, t275: F, t276: F, t902: F, t273: F, t240: F, t696: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t910 = -t903 - F::cast_from(0.17808333333333333333e-1_f64) * t908;
    let t912 = F::cast_from(0.621814e-1_f64) * t910 * t291;
    let t913 = t287 * t287;
    let t914 = F::cast_from(1.0_f64) / t913;
    let t915 = t275 * t914;
    let t916 = F::cast_from(1.0_f64) / t276;
    let t918 = -t902 / F::cast_from(3.0_f64) - t908 / F::cast_from(3.0_f64);
    let t919 = t916 * t918;
    let t921 = F::cast_from(0.29896666666666666667e0_f64) * t902;
    let t923 = F::sqrt(t273);
    let t924 = t923 * t918;
    let t926 = t696 * t240;
    (t910, t912, t913, t914, t915, t916, t918, t919, t921, t923, t924, t926)
}
