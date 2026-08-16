//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 918/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk918(t467: f64, t8020: f64, t1218: f64, t2840: f64, t4688: f64, t4711: f64, t4714: f64, t4718: f64, t4803: f64, t4807: f64, t4815: f64, t8008: f64, t8009: f64, t8011: f64, t8013: f64, t8015: f64, t8017: f64, t8019: f64) -> (f64, f64, f64) {
    let t8021 = t8020 * t467;
    let t8022 = 0.36623110073506319882e-3_f64 * t8021;
    let t8023 = t2840 * t1218;
    let t8024 = 0.11696446794910408142e1_f64 * t8023;
    let t8025 = -t4803 + t4807 + t8008 - t4815 + t4688 + t4711 - t4714 - t4718 - t8009 + t8011 - t8013 + t8015 - t8017 - t8019 - t8022 + t8024;
    (t8022, t8024, t8025)
}
