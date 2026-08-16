//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1216/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1216(t36194: f64, t36198: f64, t36205: f64, t36207: f64, t36210: f64, t36214: f64, t36231: f64, t31793: f64, t31797: f64, t31808: f64, t32942: f64, t36186: f64, t36190: f64, t36202: f64, t36217: f64, t36220: f64, t36225: f64, t36227: f64) -> f64 {
    let t37904 = 0.62896184579208304136e-2_f64 * t36194;
    let t37905 = 0.94344276868812456204e-2_f64 * t36198;
    let t37907 = 0.36675e0_f64 * t36205;
    let t37908 = 0.183375e0_f64 * t36207;
    let t37909 = 0.183375e0_f64 * t36210;
    let t37910 = 0.183375e0_f64 * t36214;
    let t37918 = 0.90702367218671976884e-1_f64 * t36231;
    let t37919 = -0.37737710747524982482e-1_f64 * t36186 + 0.56606566121287473722e-1_f64 * t36190 - t37904 + t37905 - 0.94344276868812456207e-3_f64 * t36202 - t37907 - t37908 - t37909 - t37910 + t36217 / 48.0_f64 + 0.21437009059034868486e-2_f64 * t31793 - t36220 / 6.0_f64 + 0.183375e0_f64 * t36225 - t36227 / 4.0_f64 - 0.62896184579208304138e-3_f64 * t31797 - t32942 - 0.1528125e-1_f64 * t31808 - t37918;
    t37919
}
