//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 470/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk470<F: Float>(t2846: F, t273: F, t240: F, t68: F, t281: F, t283: F, t698: F, t931: F, t1014: F, t913: F, t275: F, t290: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2892 = F::cast_from(0.39862222222222222223e0_f64) * t2846;
    let t2897 = F::cast_from(1.0_f64)/F::sqrt(t273);
    let t2902 = t68 * t240;
    let t2904 = t281 * t2902 * t283;
    let t2905 = F::cast_from(0.13692777777777777778e0_f64) * t2904;
    let t2906 = t698 * t931;
    let t2908 = t240 * t1014;
    let t2922 = t913 * t913;
    let t2923 = F::cast_from(1.0_f64) / t2922;
    let t2924 = t275 * t2923;
    let t2925 = t290 * t290;
    (t2892, t2897, t2902, t2904, t2905, t2906, t2908, t2924, t2925)
}
