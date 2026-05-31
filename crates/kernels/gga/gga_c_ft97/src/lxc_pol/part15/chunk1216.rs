//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1216/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1216<F: Float>(t82182: F, t992: F, t10916: F, t1268: F, t14519: F, t2265: F, t231: F, t2917: F, t2918: F, t54456: F, t631: F, t69374: F, t69468: F, t69510: F, t82082: F, t82267: F, t82303: F, t82326: F, t82328: F, t86571: F, t88239: F, t88252: F, t893: F, t898: F, t900: F, t91290: F, t91307: F) -> (F, F) {
    let t91330 = t82182 * t992;
    let t91334 = -F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t54456 + t631 * t231 * t893 * t86571 / F::cast_from(6.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t82267 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t69374 + t631 * t898 * t900 * (t91290 + t91307) / F::cast_from(2.0_f64) - F::cast_from(6.0_f64) * t631 * t898 * t82082 * t1268 + F::cast_from(2.0_f64) * t631 * t2917 * t10916 * t88252 + t631 * t2917 * t2918 * t88239 / F::cast_from(6.0_f64) + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t82303 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t69468 - F::cast_from(8.0_f64) * t82326 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t82328 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t69510 + F::cast_from(8.0_f64) * t2265 * t14519 * t91330;
    (t91330, t91334)
}
