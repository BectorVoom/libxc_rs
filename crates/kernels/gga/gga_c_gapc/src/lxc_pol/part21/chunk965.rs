//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 965/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk965<F: Float>(t33614: F, t33617: F, t33621: F, t33625: F, t33628: F, t33631: F, t33634: F, t33637: F, t33641: F, t33645: F, t33648: F, t11794: F, t7927: F, t9554: F, t126: F, t671: F) -> (F, F, F) {
    let t33650 = -0.34842871069624090849e-4 * t33614 + 0.4834058140556728127e-8 * t33617 - 0.51290949884214629949e-9 * t33621 - 0.10110318318802209383e-5 * t33625 - 0.10110318318802209383e-5 * t33628 - 0.2318836277704281739e-4 * t33631 + 0.17376185052903442709e-3 * t33634 + 0.34752370105806885418e-3 * t33637 - 0.2318836277704281739e-4 * t33641 - 0.10821235962619981449e-3 * t33645 + 0.34752370105806885418e-3 * t33648;
    let t33653 = t11794 * t7927 * t9554;
    let t33655 = t126 * t671;
    (t33650, t33653, t33655)
}
