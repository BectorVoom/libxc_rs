//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1315/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1315(t1020: f64, t8047: f64, t93059: f64, t26671: f64, t27845: f64, t4994: f64, t26685: f64, t26692: f64, t26748: f64, t27808: f64, t27812: f64, t27904: f64, t283: f64, t32896: f64, t95649: f64, t95865: f64, t96173: f64, t96178: f64, t96181: f64, t96184: f64, t96190: f64, t990: f64) -> (f64, f64, f64) {
    let t96193 = t1020 * t93059 * t8047;
    let t96196 = t4994 * t26671 * t27845;
    let t96200 = -0.37134344353515625e-4_f64 * t32896 * t283 * t990 * t27808 + 0.74138888888888888889e-2_f64 * t26692 * t27808 - 0.92673611111111111113e-3_f64 * t96173 - 0.92754700520833333333e-4_f64 * t26685 * t95649 + 0.16581944444444444444e-2_f64 * t96178 - 0.55273148148148148147e-3_f64 * t96181 - 0.24872916666666666666e-2_f64 * t96184 - 0.92673611111111111112e-3_f64 * t26748 * t27904 - 0.33163888888888888888e-2_f64 * t96190 - 0.88437037037037037034e-2_f64 * t96193 - 0.17687407407407407407e-1_f64 * t96196 + 0.185671721767578125e-4_f64 * t27812 * t95865;
    (t96193, t96196, t96200)
}
