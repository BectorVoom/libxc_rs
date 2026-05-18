//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 234/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk234<F: Float>(t287: F, t275: F, t276: F, t902: F, t273: F, t240: F, t696: F, t281: F, t283: F, t346: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t913 = t287 * t287;
    let t914 = F::new(1.0) / t913;
    let t915 = t275 * t914;
    let t916 = F::new(1.0) / t276;
    let t921 = F::new(0.29896666666666666667e0) * t902;
    let t923 = f64::sqrt(t273);
    let t926 = t696 * t240;
    let t928 = t281 * t926 * t283;
    let t929 = F::new(0.82156666666666666667e-1) * t928;
    let t930 = t240 * t346;
    (t913, t914, t915, t916, t921, t923, t926, t928, t929, t930)
}
