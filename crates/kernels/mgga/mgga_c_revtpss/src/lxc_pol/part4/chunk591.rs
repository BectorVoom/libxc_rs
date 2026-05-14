//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 591/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk591<F: Float>(t2875: F, t935: F, t2874: F, t273: F, t276: F, t918: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t916: F, t923: F, t240: F, t68: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2876 = t2875 * t935;
    let t2878 = 2.0 * t2874 * t2876;
    let t2880 = 1.0 / t276 / t273;
    let t2881 = t918 * t918;
    let t2882 = t2880 * t2881;
    let t2884 = 4.0 / 9.0 * t2846;
    let t2889 = t2884 + 2.0 / 9.0 * t2848 - 2.0 / 9.0 * t2855 + 2.0 / 3.0 * t2860 - t2864 / 3.0;
    let t2890 = t916 * t2889;
    let t2892 = 0.39862222222222222223e0 * t2846;
    let t2897 = 1.0/f64::sqrt(t273);
    let t2898 = t2897 * t2881;
    let t2900 = t923 * t2889;
    let t2902 = t68 * t240;
    let t2904 = t281 * t2902 * t283;
    (t2876, t2878, t2880, t2881, t2882, t2884, t2889, t2890, t2892, t2897, t2898, t2900, t2902, t2904)
}
