//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1123/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1123<F: Float>(t33457: F, t10840: F, t15362: F, t10867: F, t22405: F, t33378: F, t825: F, t969: F, t2464: F, t2465: F, t8528: F, t10837: F, t7416: F, t24751: F, t2676: F, t24945: F) -> (F, F, F, F, F, F, F, F) {
    let t33458 = 0.2698205900461089792e0 * t33457;
    let t33459 = t15362 * t10840;
    let t33460 = 0.59584149919750711116e-1 * t33459;
    let t33461 = t10867 * t22405;
    let t33462 = 0.44688112439813033337e-1 * t33461;
    let t33464 = t825 * t969 * t33378;
    let t33465 = 0.19171462976960374838e0 * t33464;
    let t33468 = t825 * t2464 * t2465 * t8528;
    let t33469 = 0.85206502119823888168e-1 * t33468;
    let t33473 = t7416 * t10837;
    let t33474 = 0.51123901271894332902e0 * t33473;
    let t33476 = 0.23833659967900284446e0 * t24751 * t2676;
    let t33478 = 0.23833659967900284446e0 * t24945 * t2676;
    (t33458, t33460, t33462, t33465, t33469, t33474, t33476, t33478)
}
