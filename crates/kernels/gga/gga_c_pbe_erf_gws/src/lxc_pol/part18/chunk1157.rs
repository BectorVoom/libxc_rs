//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1157/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1157<F: Float>(t11750: F, t51351: F, t11444: F, t11938: F, t14498: F, t51201: F, t54015: F, t56883: F, t56885: F, t56887: F, t56889: F, t56892: F, t56894: F, t56896: F, t56898: F, t11427: F, t51306: F) -> (F, F) {
    let t56901 = t51351 * t11750;
    let t56903 = t51351 * t11444;
    let t56905 = t14498 * t11938;
    let t56907 = -t56883 / 96.0 - t56885 / 96.0 - t56887 / 192.0 + t56889 / 48.0 - t56892 / 48.0 + t56894 / 384.0 + t56896 / 256.0 + t54015 - t56898 / 192.0 + 119.0 / 3456.0 * t51201 - t56901 / 192.0 - t56903 / 96.0 - t56905 / 64.0;
    let t56910 = t51306 * t11427;
    (t56907, t56910)
}
