//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1320/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1320<F: Float>(t11521: F, t14498: F, t11930: F, t14015: F, t11750: F, t51351: F, t11444: F, t11938: F, t51201: F, t54015: F, t56883: F, t56885: F, t56887: F, t56889: F, t56892: F, t56894: F) -> F {
    let t56896 = t14498 * t11521;
    let t56898 = t14015 * t11930;
    let t56901 = t51351 * t11750;
    let t56903 = t51351 * t11444;
    let t56905 = t14498 * t11938;
    let t56907 = -t56883 / F::cast_from(96.0_f64) - t56885 / F::cast_from(96.0_f64) - t56887 / F::cast_from(192.0_f64) + t56889 / F::cast_from(48.0_f64) - t56892 / F::cast_from(48.0_f64) + t56894 / F::cast_from(384.0_f64) + t56896 / F::cast_from(256.0_f64) + t54015 - t56898 / F::cast_from(192.0_f64) + F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t51201 - t56901 / F::cast_from(192.0_f64) - t56903 / F::cast_from(96.0_f64) - t56905 / F::cast_from(64.0_f64);
    t56907
}
