//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 642/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk642<F: Float>(t1583: F, t5255: F, t1582: F, t1492: F, t4305: F, t1220: F, t1579: F, t1588: F, t277: F, t2841: F, t2911: F, t4216: F, t4230: F, t4297: F, t4536: F, t5087: F, t5098: F, t5103: F, t5167: F, t5226: F, t5229: F, t5233: F, t5243: F, t5246: F, t5250: F, t95: F) -> (F, F, F, F) {
    let t5256 = t1583 * t5255;
    let t5257 = t1582 * t5256;
    let t5261 = F::cast_from(0.11696446794910408142e1_f64) * t4305 * t1492;
    let t5262 = -F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t5087 * t2911 + t4536 * t1579 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4230 * t1579 + t1220 * t5098 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1220 * t5103 + t5226 + F::cast_from(100.0_f64) / F::cast_from(81.0_f64) * t4216 + F::cast_from(100.0_f64) / F::cast_from(81.0_f64) * t4297 * t5229 - t1220 * t5233 / F::cast_from(3.0_f64) + F::cast_from(20000.0_f64) / F::cast_from(81.0_f64) * t5243 * t5246 + F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t5250 * t1588 - F::cast_from(50.0_f64) / F::cast_from(3.0_f64) * t5257 * t1588 - t2841 - t5167 - t5261;
    (t5256, t5257, t5261, t5262)
}
