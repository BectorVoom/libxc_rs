//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1165/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1165<F: Float>(t33570: F, t33614: F, t33617: F, t33621: F, t33625: F, t33628: F, t33631: F, t33634: F, t33637: F, t33641: F, t33645: F, t33648: F, t33576: F, t33578: F, t33580: F, t33583: F, t33585: F, t33588: F, t33590: F, t33595: F, t33601: F, t33606: F) -> (F, F) {
    let t36631 = 0.6070699179094394313e-6 * t33570;
    let t36657 = -0.69685742139248181696e-4 * t33614 + 0.96681162811134562538e-8 * t33617 - 0.1025818997684292599e-8 * t33621 - 0.20220636637604418766e-5 * t33625 - 0.20220636637604418766e-5 * t33628 - 0.4637672555408563478e-4 * t33631 + 0.34752370105806885418e-3 * t33634 + 0.69504740211613770836e-3 * t33637 - 0.4637672555408563478e-4 * t33641 - 0.21642471925239962898e-3 * t33645 + 0.69504740211613770836e-3 * t33648;
    let t36658 = -0.10793703140429833089e-5 * t33576 - 0.12141398358188788626e-5 * t33578 + 0.21587406280859666178e-5 * t33580 + 0.10551281119038438161e-7 * t33583 + 0.21102562238076876322e-7 * t33585 - 0.22509399720615334744e-6 * t33588 + 0.12817159869818982005e-5 * t33590 + 0.20220636637604418766e-5 * t33595 - 0.15018333275585850553e-5 * t33601 - 0.20220636637604418766e-4 * t33606 + t36657;
    (t36631, t36658)
}
