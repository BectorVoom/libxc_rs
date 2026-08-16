//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1216/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1216(t82182: f64, t992: f64, t10916: f64, t1268: f64, t14519: f64, t2265: f64, t231: f64, t2917: f64, t2918: f64, t54456: f64, t631: f64, t69374: f64, t69468: f64, t69510: f64, t82082: f64, t82267: f64, t82303: f64, t82326: f64, t82328: f64, t86571: f64, t88239: f64, t88252: f64, t893: f64, t898: f64, t900: f64, t91290: f64, t91307: f64) -> (f64, f64) {
    let t91330 = t82182 * t992;
    let t91334 = -160.0_f64 / 81.0_f64 * t54456 + t631 * t231 * t893 * t86571 / 6.0_f64 - 4.0_f64 / 3.0_f64 * t82267 + 10.0_f64 / 3.0_f64 * t69374 + t631 * t898 * t900 * (t91290 + t91307) / 2.0_f64 - 6.0_f64 * t631 * t898 * t82082 * t1268 + 2.0_f64 * t631 * t2917 * t10916 * t88252 + t631 * t2917 * t2918 * t88239 / 6.0_f64 + 8.0_f64 / 3.0_f64 * t82303 - 40.0_f64 / 9.0_f64 * t69468 - 8.0_f64 * t82326 + 8.0_f64 / 9.0_f64 * t82328 + 10.0_f64 / 27.0_f64 * t69510 + 8.0_f64 * t2265 * t14519 * t91330;
    (t91330, t91334)
}
