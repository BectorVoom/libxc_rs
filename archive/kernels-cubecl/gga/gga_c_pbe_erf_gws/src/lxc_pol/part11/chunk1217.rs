//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1217/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1217<F: Float>(t44465: F, t27197: F, t11773: F, t11778: F, t2121: F, t27556: F, t337: F, t49022: F, t5: F, t3180: F, t45579: F, t13156: F, t3116: F) -> (F, F, F, F, F, F) {
    let t49315 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t44465;
    let t49316 = F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t27197;
    let t49318 = t11773 * t11778 / F::cast_from(16.0_f64);
    let t49327 = t27556 * t2121 * t337 * t5 * t49022 / F::cast_from(16.0_f64);
    let t49329 = t45579 * t3180 / F::cast_from(12.0_f64);
    let t49334 = t3116 * t2121 * t337 * t5 * t13156 / F::cast_from(96.0_f64);
    (t49315, t49316, t49318, t49327, t49329, t49334)
}
