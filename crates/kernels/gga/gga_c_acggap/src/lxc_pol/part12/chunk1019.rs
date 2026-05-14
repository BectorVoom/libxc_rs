//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1019/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1019<F: Float>(t34361: F, t30365: F, t30369: F, t30375: F, t30387: F, t30398: F, t30412: F, t30416: F, t30444: F, t30448: F, t30452: F, t30457: F, t30459: F, t32456: F, t32458: F, t32461: F, t32462: F, t34371: F) -> (F,) {
    let t37047 = 0.25724410870841842184e-1 * t34361;
    let t37058 = -0.17149607247227894789e-2 * t30365 + 0.41930789719472202759e-2 * t30369 + 0.25158473831683321656e-2 * t30375 - t37047 + 11.0 / 192.0 * t30387 - t32456 + 35.0 / 108.0 * t30398 - t32458 + 0.12579236915841660828e-1 * t30412 - 0.50316947663366643309e-2 * t30416 + t32461 + 0.36675e0 * t34371 + t32462 - 0.31448092289604152068e-2 * t30444 - 0.12862205435420921092e-2 * t30448 + 0.12579236915841660828e-2 * t30452 - 0.18007087609589289529e-1 * t30457 + 0.85748036236139473944e-3 * t30459;
    (t37058,)
}
