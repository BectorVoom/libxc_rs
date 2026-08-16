//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 878/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk878<F: Float>(t2411: F, t52: F, t154: F, t6406: F, t2380: F, t3174: F, t3185: F, t3206: F, t3235: F, t385: F, t404: F, t6369: F, t6373: F, t6379: F, t6383: F, t6387: F, t6392: F, t6395: F, t6401: F, t6408: F, t6413: F, t6419: F, t6424: F, t6430: F, t6434: F) -> (F, F) {
    let t6436 = t52 * t2411;
    let t6438 = t154 * t6436 * t6406;
    let t6441 = F::cast_from(0.38586616306262763275e-2_f64) * t2380 * t6369 + F::cast_from(0.12862205435420921092e-2_f64) * t3206 * t6373 + t6379 + F::cast_from(0.28582678745379824648e-3_f64) * t6383 + F::cast_from(0.38586616306262763276e-2_f64) * t3235 * t6387 - F::cast_from(0.85748036236139473944e-3_f64) * t6392 - F::cast_from(0.42874018118069736972e-3_f64) * t404 * t6395 + F::cast_from(0.25724410870841842184e-2_f64) * t6401 - F::cast_from(0.51448821741683684368e-2_f64) * t404 * t6408 - F::cast_from(0.25724410870841842183e-2_f64) * t3185 * t6413 + F::cast_from(0.12862205435420921092e-2_f64) * t3185 * t6419 + t3174 * t6424 / F::cast_from(16.0_f64) + t6430 + t6434 / F::cast_from(48.0_f64) - t385 * t6438 / F::cast_from(16.0_f64);
    (t6438, t6441)
}
