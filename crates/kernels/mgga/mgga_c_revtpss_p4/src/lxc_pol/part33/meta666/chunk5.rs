//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2184/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2184<F: Float>(t6864: F, t94455: F, t26024: F, t6846: F, t102529: F, t102549: F, t94484: F, t94498: F, t98222: F, t98227: F, t98230: F, t98236: F, t98239: F, t98244: F, t98259: F) -> F {
    let t108590 = t94455 * t6864;
    let t108592 = t26024 * t6846;
    let t108596 = -F::cast_from(0.40015750243531754507e-2_f64) * t108590 + F::cast_from(0.20007875121765877254e-2_f64) * t108592 - F::cast_from(0.80031500487063509015e-1_f64) * t98222 - t102529 + t98227 - t98230 - t98236 + t98239 + t94484 + t98244 + F::cast_from(0.27104001498285508387e-3_f64) * t94498 - t98259 - t102549;
    t108596
}
