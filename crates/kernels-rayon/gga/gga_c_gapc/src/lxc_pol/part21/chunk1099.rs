//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1099/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1099(t189: f64, t615: f64, t11749: f64, t933: f64, t11790: f64, t3367: f64, t6188: f64, t33614: f64, t33617: f64, t33621: f64, t33625: f64, t33628: f64, t33631: f64, t33634: f64, t33637: f64, t33641: f64) -> (f64, f64) {
    let t33643 = t189 * t615;
    let t33645 = t933 * t33643 * t11749;
    let t33648 = t11790 * t3367 * t6188;
    let t33650 = -0.34842871069624090849e-4_f64 * t33614 + 0.4834058140556728127e-8_f64 * t33617 - 0.51290949884214629949e-9_f64 * t33621 - 0.10110318318802209383e-5_f64 * t33625 - 0.10110318318802209383e-5_f64 * t33628 - 0.2318836277704281739e-4_f64 * t33631 + 0.17376185052903442709e-3_f64 * t33634 + 0.34752370105806885418e-3_f64 * t33637 - 0.2318836277704281739e-4_f64 * t33641 - 0.10821235962619981449e-3_f64 * t33645 + 0.34752370105806885418e-3_f64 * t33648;
    (t33643, t33650)
}
