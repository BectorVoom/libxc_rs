//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1217/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1217(t44465: f64, t27197: f64, t11773: f64, t11778: f64, t2121: f64, t27556: f64, t337: f64, t49022: f64, t5: f64, t3180: f64, t45579: f64, t13156: f64, t3116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49315 = 7.0_f64 / 72.0_f64 * t44465;
    let t49316 = 455.0_f64 / 324.0_f64 * t27197;
    let t49318 = t11773 * t11778 / 16.0_f64;
    let t49327 = t27556 * t2121 * t337 * t5 * t49022 / 16.0_f64;
    let t49329 = t45579 * t3180 / 12.0_f64;
    let t49334 = t3116 * t2121 * t337 * t5 * t13156 / 96.0_f64;
    (t49315, t49316, t49318, t49327, t49329, t49334)
}
