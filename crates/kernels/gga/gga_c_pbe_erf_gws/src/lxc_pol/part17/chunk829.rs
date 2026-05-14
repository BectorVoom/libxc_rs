//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 829/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk829<F: Float>(t587: F, t7941: F, t5557: F, t1023: F, t1672: F, t616: F, t2786: F, t579: F, t202: F, t2814: F, t184: F, t619: F, t1735: F, t2741: F, t996: F, t561: F) -> (F, F, F, F, F, F, F) {
    let t7942 = t587 * t7941;
    let t7943 = 8.0 / 27.0 * t7942;
    let t7944 = 16.0 / 135.0 * t5557;
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7947 = 8.0 / 135.0 * t7946;
    let t7949 = 4.0 / 15.0 * t579 * t2786;
    let t7950 = t202 * t2814;
    let t7951 = t7950 * t184;
    let t7953 = 8.0 / 15.0 * t7951 * t619;
    let t7955 = 4.0 / 15.0 * t2741 * t1735;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    (t7943, t7944, t7947, t7949, t7953, t7955, t7957)
}
