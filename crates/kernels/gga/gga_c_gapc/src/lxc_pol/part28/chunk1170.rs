//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1170/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1170<F: Float>(t33563: F, t33567: F, t33570: F, t33541: F, t33552: F, t33565: F, t36621: F, t36623: F, t36625: F, t36626: F, t36627: F, t33614: F, t33617: F, t33621: F, t33625: F, t33628: F, t33631: F, t33634: F, t33637: F, t33641: F, t33645: F, t33648: F) -> (F, F) {
    let t36628 = 0.64085799349094910026e-6 * t33563;
    let t36630 = 0.54924190264999682021e-4 * t33567;
    let t36631 = 0.6070699179094394313e-6 * t33570;
    let t36632 = t36621 - 0.53808777420609085653e-7 * t33541 + t36623 - 0.89048050908546122981e-5 * t33552 - t36625 + t36626 - t36627 - t36628 + 0.12650553385416666668e-5 * t33565 + t36630 + t36631;
    let t36657 = -0.69685742139248181696e-4 * t33614 + 0.96681162811134562538e-8 * t33617 - 0.1025818997684292599e-8 * t33621 - 0.20220636637604418766e-5 * t33625 - 0.20220636637604418766e-5 * t33628 - 0.4637672555408563478e-4 * t33631 + 0.34752370105806885418e-3 * t33634 + 0.69504740211613770836e-3 * t33637 - 0.4637672555408563478e-4 * t33641 - 0.21642471925239962898e-3 * t33645 + 0.69504740211613770836e-3 * t33648;
    (t36632, t36657)
}
