//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1760/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1760<F: Float>(t90400: F, t90456: F, t90478: F, t90497: F, t1179: F, t1188: F, t1196: F, t6474: F, t68952: F, t90349: F, t90351: F, t90356: F, t90361: F, t90364: F, t90367: F, t90370: F, t90373: F, t90375: F, t90377: F) -> (F, F, F, F) {
    let t90499 = t90400 + t90456 + t90478 + t90497;
    let t90503 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t1179 * t90499 * t1188;
    let t90505 = F::cast_from(0.96491876992155210402e2_f64) * t68952 * t6474;
    let t90506 = -t90349 + t90351 - t90356 - t90361 - t90364 - t90367 + t90370 + t90373 - t90375 - t90377 - t90503 + t90505;
    (t90499, t90503, t90505, t90506)
}
