//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 559/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk559<F: Float>(t910: F, t914: F, t287: F, t913: F, t275: F, t273: F, t276: F, t2846: F, t240: F, t68: F, t281: F, t283: F, t698: F, t931: F, t1014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2869 = t910 * t914;
    let t2872 = t913 * t287;
    let t2873 = 1.0 / t2872;
    let t2874 = t275 * t2873;
    let t2880 = 1.0 / t276 / t273;
    let t2884 = 4.0 / 9.0 * t2846;
    let t2892 = 0.39862222222222222223e0 * t2846;
    let t2897 = 1.0/f64::sqrt(t273);
    let t2902 = t68 * t240;
    let t2904 = t281 * t2902 * t283;
    let t2905 = 0.13692777777777777778e0 * t2904;
    let t2906 = t698 * t931;
    let t2908 = t240 * t1014;
    (t2869, t2873, t2874, t2880, t2884, t2892, t2897, t2902, t2904, t2905, t2906, t2908)
}
