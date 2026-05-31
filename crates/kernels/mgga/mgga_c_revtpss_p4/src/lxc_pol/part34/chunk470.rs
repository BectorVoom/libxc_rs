//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 470/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk470<F: Float>(t287: F, t913: F, t275: F, t273: F, t276: F, t2846: F, t240: F, t68: F, t281: F, t283: F, t1014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2872 = t913 * t287;
    let t2873 = F::cast_from(1.0_f64) / t2872;
    let t2874 = t275 * t2873;
    let t2880 = F::cast_from(1.0_f64) / t276 / t273;
    let t2884 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2846;
    let t2892 = F::cast_from(0.39862222222222222223e0_f64) * t2846;
    let t2897 = F::cast_from(1.0_f64)/F::sqrt(t273);
    let t2902 = t68 * t240;
    let t2904 = t281 * t2902 * t283;
    let t2905 = F::cast_from(0.13692777777777777778e0_f64) * t2904;
    let t2908 = t240 * t1014;
    let t2922 = t913 * t913;
    let t2923 = F::cast_from(1.0_f64) / t2922;
    (t2873, t2874, t2880, t2884, t2892, t2897, t2902, t2904, t2905, t2908, t2922, t2923)
}
