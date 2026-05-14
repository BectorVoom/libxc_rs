//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1043/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1043<F: Float>(t4258: F, t8806: F, t30248: F, t532: F, t31793: F, t31797: F, t31806: F, t31808: F, t36186: F, t36190: F, t36195: F, t36199: F, t36202: F, t36206: F, t36208: F, t36211: F, t36215: F, t36217: F, t36220: F, t36225: F) -> (F,) {
    let t36227 = t8806 * t4258;
    let t36231 = t30248 * t532;
    let t36233 = -0.18868855373762491241e-1 * t36186 + 0.28303283060643736862e-1 * t36190 - t36195 + t36199 - 0.47172138434406228102e-3 * t36202 - t36206 - t36208 - t36211 - t36215 + t36217 / 96.0 + 0.10718504529517434243e-2 * t31793 - t36220 / 12.0 + 0.916875e-1 * t36225 - t36227 / 8.0 - 0.31448092289604152068e-3 * t31797 - t31806 - 0.7640625e-2 * t31808 - 0.45351183609335988442e-1 * t36231;
    (t36233,)
}
