//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1373/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1373(t33570: f64, t33614: f64, t33617: f64, t33621: f64, t33625: f64, t33628: f64, t33631: f64, t33634: f64, t33637: f64, t33641: f64, t33645: f64, t33648: f64) -> (f64, f64) {
    let t36631 = 0.6070699179094394313e-6_f64 * t33570;
    let t36657 = -0.69685742139248181696e-4_f64 * t33614 + 0.96681162811134562538e-8_f64 * t33617 - 0.1025818997684292599e-8_f64 * t33621 - 0.20220636637604418766e-5_f64 * t33625 - 0.20220636637604418766e-5_f64 * t33628 - 0.4637672555408563478e-4_f64 * t33631 + 0.34752370105806885418e-3_f64 * t33634 + 0.69504740211613770836e-3_f64 * t33637 - 0.4637672555408563478e-4_f64 * t33641 - 0.21642471925239962898e-3_f64 * t33645 + 0.69504740211613770836e-3_f64 * t33648;
    (t36631, t36657)
}
