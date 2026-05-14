//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 536/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk536<F: Float>(t120: F, t2873: F, t102: F, t156: F, t974: F, t496: F, t481: F, t978: F, t128: F, t10: F, t501: F, t395: F, t1563: F, t967: F, t506: F, t127: F, t1511: F, t1519: F, t1540: F, t1542: F, t1555: F, t1558: F, t1561: F, t2862: F, t2865: F, t2868: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2874 = t120 * t2873;
    let t2876 = 0.2923025e1 * t102 * t2874;
    let t2878 = t156 * t974;
    let t2879 = t496 * t2878;
    let t2881 = t978 * t481;
    let t2885 = t128 * t2873;
    let t2886 = t10 * t2885;
    let t2890 = t501 * t978;
    let t2891 = t2890 * t395;
    let t2893 = t1563 * t967;
    let t2897 = t506 * t2873;
    let t2900 = -t1511 + t2862 + t1519 + t2865 + t2868 - t2876 + t1540 + t1542 / 6.0 + t2879 / 6.0 + 3.0 / 2.0 * t496 * t10 * t2881 - t496 * t2886 / 2.0 + t1555 + 0.73452e0 * t1558 + t1561 + 0.73452e0 * t2891 + 0.587616e1 * t127 * t2893 * t481 - 0.146904e1 * t127 * t2897;
    (t2874, t2876, t2878, t2879, t2881, t2885, t2886, t2890, t2891, t2893, t2897, t2900)
}
