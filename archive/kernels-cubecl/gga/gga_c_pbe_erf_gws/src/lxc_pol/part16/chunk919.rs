//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 919/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk919<F: Float>(t467: F, t8020: F, t1218: F, t2840: F, t4688: F, t4711: F, t4714: F, t4718: F, t4803: F, t4807: F, t4815: F, t8008: F, t8009: F, t8011: F, t8013: F, t8015: F, t8017: F, t8019: F) -> (F, F, F) {
    let t8021 = t8020 * t467;
    let t8022 = F::cast_from(0.36623110073506319882e-3_f64) * t8021;
    let t8023 = t2840 * t1218;
    let t8024 = F::cast_from(0.11696446794910408142e1_f64) * t8023;
    let t8025 = -t4803 + t4807 + t8008 - t4815 + t4688 + t4711 - t4714 - t4718 - t8009 + t8011 - t8013 + t8015 - t8017 - t8019 - t8022 + t8024;
    (t8022, t8024, t8025)
}
