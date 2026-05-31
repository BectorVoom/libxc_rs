//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1185/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1185<F: Float>(t4264: F, t7436: F, t142: F, t3706: F, t1017: F, t2060: F, t2288: F, t4258: F, t8806: F, t30248: F, t532: F, t31793: F, t31797: F, t31806: F, t31808: F, t36186: F, t36190: F, t36195: F, t36199: F, t36202: F, t36206: F, t36208: F, t36211: F, t36215: F, t36217: F) -> F {
    let t36220 = t7436 * t4264;
    let t36222 = t142 * t3706;
    let t36225 = t2060 * t36222 * t2288 * t1017;
    let t36227 = t8806 * t4258;
    let t36231 = t30248 * t532;
    let t36233 = -F::cast_from(0.18868855373762491241e-1_f64) * t36186 + F::cast_from(0.28303283060643736862e-1_f64) * t36190 - t36195 + t36199 - F::cast_from(0.47172138434406228102e-3_f64) * t36202 - t36206 - t36208 - t36211 - t36215 + t36217 / F::cast_from(96.0_f64) + F::cast_from(0.10718504529517434243e-2_f64) * t31793 - t36220 / F::cast_from(12.0_f64) + F::cast_from(0.916875e-1_f64) * t36225 - t36227 / F::cast_from(8.0_f64) - F::cast_from(0.31448092289604152068e-3_f64) * t31797 - t31806 - F::cast_from(0.7640625e-2_f64) * t31808 - F::cast_from(0.45351183609335988442e-1_f64) * t36231;
    t36233
}
