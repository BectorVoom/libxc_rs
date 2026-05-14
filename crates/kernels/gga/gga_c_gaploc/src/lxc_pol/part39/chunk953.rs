//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 953/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk953<F: Float>(t313: F, t314: F, t317: F, t47311: F, t13876: F, t2197: F, t568: F, t833: F, t836: F, t47187: F, t701: F, t1457: F, t2004: F, t13870: F, t795: F, t2639: F) -> (F, F, F, F, F, F, F) {
    let t47315 = 0.35750489951850426669e0 * t313 * t314 * t47311 * t317;
    let t47317 = 0.23005755572352449806e1 * t2197 * t13876;
    let t47321 = 0.23005755572352449806e1 * t833 * t568 * t836 * t47311;
    let t47322 = t47187 * t701;
    let t47325 = 0.35750489951850426669e0 * t2004 * t1457 * t47322;
    let t47326 = t795 * t13870;
    let t47327 = t313 * t47326;
    let t47329 = 0.10725146985555128001e1 * t47327 * t2639;
    (t47315, t47317, t47321, t47322, t47325, t47326, t47329)
}
