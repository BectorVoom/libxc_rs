//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 595/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk595<F: Float>(t159: F, t2522: F, t285: F, t545: F, t991: F, t281: F, t1083: F, t751: F, t164: F, t2519: F, t547: F, t992: F, t331: F, t551: F, t553: F, t1052: F, t163: F, t169: F, t299: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2932 = t2522 * t159 * t285;
    let t2936 = t991 * t545 * t285;
    let t2937 = t281 * t2936;
    let t2939 = t751 * t1083;
    let t2942 = t2519 * t164;
    let t2946 = t992 * t547;
    let t2948 = t331 * t991;
    let t2950 = t2948 * t551 * t553;
    let t2957 = t169 * t299 * t1052 * t163;
    (t2932, t2936, t2937, t2939, t2942, t2946, t2948, t2950, t2957)
}
