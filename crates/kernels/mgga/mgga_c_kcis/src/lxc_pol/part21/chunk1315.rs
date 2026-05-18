//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1315/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1315<F: Float>(t1020: F, t8047: F, t93059: F, t26671: F, t27845: F, t4994: F, t26685: F, t26692: F, t26748: F, t27808: F, t27812: F, t27904: F, t283: F, t32896: F, t95649: F, t95865: F, t96173: F, t96178: F, t96181: F, t96184: F, t96190: F, t990: F) -> (F, F, F) {
    let t96193 = t1020 * t93059 * t8047;
    let t96196 = t4994 * t26671 * t27845;
    let t96200 = -F::new(0.37134344353515625e-4) * t32896 * t283 * t990 * t27808 + F::new(0.74138888888888888889e-2) * t26692 * t27808 - F::new(0.92673611111111111113e-3) * t96173 - F::new(0.92754700520833333333e-4) * t26685 * t95649 + F::new(0.16581944444444444444e-2) * t96178 - F::new(0.55273148148148148147e-3) * t96181 - F::new(0.24872916666666666666e-2) * t96184 - F::new(0.92673611111111111112e-3) * t26748 * t27904 - F::new(0.33163888888888888888e-2) * t96190 - F::new(0.88437037037037037034e-2) * t96193 - F::new(0.17687407407407407407e-1) * t96196 + F::new(0.185671721767578125e-4) * t27812 * t95865;
    (t96193, t96196, t96200)
}
