//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 935/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk935(t133: f64, t8199: f64, t2911: f64, t2912: f64, t8152: f64, t8177: f64, t8181: f64, t8182: f64, t8186: f64, t8193: f64, t8198: f64, t8231: f64, t8232: f64, t8238: f64, t8240: f64, t8244: f64, t8249: f64) -> f64 {
    let t8252 = t133 * t8199;
    let t8254 = t8177 - t8181 - t8182 - 0.2069106e2_f64 * t2911 * t8231 * t8232 - t8186 - 0.344851e1_f64 * t8238 + 0.1034553e2_f64 * t2911 * t2912 * t8240 + 0.5172765e1_f64 * t2911 * t2912 * t8244 - t8193 - t8198 + t8249 - 0.1724255e1_f64 * t133 * t8152 - 0.76633555555555555556e0_f64 * t8252;
    t8254
}
