//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1096/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1096(t11808: f64, t11983: f64, t11772: f64, t29692: f64, t11795: f64, t9387: f64, t11508: f64, t3402: f64, t7944: f64, t33536: f64, t33541: f64, t33547: f64, t33552: f64, t33555: f64, t33558: f64, t33561: f64) -> f64 {
    let t33563 = t11808 * t11983;
    let t33565 = t11772 * t29692;
    let t33567 = t9387 * t11795;
    let t33570 = t3402 * t11508 * t7944;
    let t33572 = 0.2209855149968790001e-7_f64 * t33536 - 0.26904388710304542825e-7_f64 * t33541 + 0.2504163411376437654e-5_f64 * t33547 - 0.44524025454273061491e-5_f64 * t33552 - 0.30353495895471971564e-6_f64 * t33555 + 0.53968515702149165444e-6_f64 * t33558 - 0.32042899674547455014e-6_f64 * t33561 - 0.32042899674547455014e-6_f64 * t33563 + 0.63252766927083333336e-6_f64 * t33565 + 0.27462095132499841011e-4_f64 * t33567 + 0.30353495895471971564e-6_f64 * t33570;
    t33572
}
