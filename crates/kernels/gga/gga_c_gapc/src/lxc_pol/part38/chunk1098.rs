//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1098/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1098<F: Float>(t189: F, t615: F, t11749: F, t933: F, t11790: F, t3367: F, t6188: F, t33614: F, t33617: F, t33621: F, t33625: F, t33628: F, t33631: F, t33634: F, t33637: F, t33641: F) -> (F, F) {
    let t33643 = t189 * t615;
    let t33645 = t933 * t33643 * t11749;
    let t33648 = t11790 * t3367 * t6188;
    let t33650 = -F::new(0.34842871069624090849e-4) * t33614 + F::new(0.4834058140556728127e-8) * t33617 - F::new(0.51290949884214629949e-9) * t33621 - F::new(0.10110318318802209383e-5) * t33625 - F::new(0.10110318318802209383e-5) * t33628 - F::new(0.2318836277704281739e-4) * t33631 + F::new(0.17376185052903442709e-3) * t33634 + F::new(0.34752370105806885418e-3) * t33637 - F::new(0.2318836277704281739e-4) * t33641 - F::new(0.10821235962619981449e-3) * t33645 + F::new(0.34752370105806885418e-3) * t33648;
    (t33643, t33650)
}
