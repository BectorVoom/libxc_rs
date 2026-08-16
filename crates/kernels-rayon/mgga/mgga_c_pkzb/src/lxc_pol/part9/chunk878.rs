//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 878/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk878(t2411: f64, t52: f64, t154: f64, t6406: f64, t2380: f64, t3174: f64, t3185: f64, t3206: f64, t3235: f64, t385: f64, t404: f64, t6369: f64, t6373: f64, t6379: f64, t6383: f64, t6387: f64, t6392: f64, t6395: f64, t6401: f64, t6408: f64, t6413: f64, t6419: f64, t6424: f64, t6430: f64, t6434: f64) -> (f64, f64) {
    let t6436 = t52 * t2411;
    let t6438 = t154 * t6436 * t6406;
    let t6441 = 0.38586616306262763275e-2_f64 * t2380 * t6369 + 0.12862205435420921092e-2_f64 * t3206 * t6373 + t6379 + 0.28582678745379824648e-3_f64 * t6383 + 0.38586616306262763276e-2_f64 * t3235 * t6387 - 0.85748036236139473944e-3_f64 * t6392 - 0.42874018118069736972e-3_f64 * t404 * t6395 + 0.25724410870841842184e-2_f64 * t6401 - 0.51448821741683684368e-2_f64 * t404 * t6408 - 0.25724410870841842183e-2_f64 * t3185 * t6413 + 0.12862205435420921092e-2_f64 * t3185 * t6419 + t3174 * t6424 / 16.0_f64 + t6430 + t6434 / 48.0_f64 - t385 * t6438 / 16.0_f64;
    (t6438, t6441)
}
