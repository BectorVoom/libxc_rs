//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1185/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1185(t4264: f64, t7436: f64, t142: f64, t3706: f64, t1017: f64, t2060: f64, t2288: f64, t4258: f64, t8806: f64, t30248: f64, t532: f64, t31793: f64, t31797: f64, t31806: f64, t31808: f64, t36186: f64, t36190: f64, t36195: f64, t36199: f64, t36202: f64, t36206: f64, t36208: f64, t36211: f64, t36215: f64, t36217: f64) -> f64 {
    let t36220 = t7436 * t4264;
    let t36222 = t142 * t3706;
    let t36225 = t2060 * t36222 * t2288 * t1017;
    let t36227 = t8806 * t4258;
    let t36231 = t30248 * t532;
    let t36233 = -0.18868855373762491241e-1_f64 * t36186 + 0.28303283060643736862e-1_f64 * t36190 - t36195 + t36199 - 0.47172138434406228102e-3_f64 * t36202 - t36206 - t36208 - t36211 - t36215 + t36217 / 96.0_f64 + 0.10718504529517434243e-2_f64 * t31793 - t36220 / 12.0_f64 + 0.916875e-1_f64 * t36225 - t36227 / 8.0_f64 - 0.31448092289604152068e-3_f64 * t31797 - t31806 - 0.7640625e-2_f64 * t31808 - 0.45351183609335988442e-1_f64 * t36231;
    t36233
}
