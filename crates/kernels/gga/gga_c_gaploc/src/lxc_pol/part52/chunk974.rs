//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 974/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk974<F: Float>(t14373: F, t14374: F, t2639: F, t313: F, t314: F, t317: F, t45606: F, t45608: F, t45611: F, t45614: F, t45617: F, t45627: F, t45630: F, t45633: F, t45636: F, t45639: F, t47377: F, t47378: F, t47379: F, t50130: F, t50182: F, t50183: F, t50194: F, t531: F, t568: F, t7427: F, t7573: F, t769: F, t784: F, t797: F, t833: F, t836: F) -> F {
    let t50208 = t45606 - t45608 - t45611 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t531 * t50183 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t568 * t836 * t50182 + F::cast_from(0.23833659967900284446e0_f64) * t14374 * t784 + t45614 - t45617 - F::cast_from(0.50050685932590597338e1_f64) * t50194 * t2639 + t47377 - t47378 - t45627 + t45630 - t45633 - F::cast_from(0.12423108009070322895e3_f64) * t7427 * t7573 * t50130 - t45636 + t45639 + F::cast_from(0.35750489951850426669e0_f64) * t769 * t14373 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t313 * t314 * t50182 * t317 + F::cast_from(0.59584149919750711116e-1_f64) * t47379;
    t50208
}
