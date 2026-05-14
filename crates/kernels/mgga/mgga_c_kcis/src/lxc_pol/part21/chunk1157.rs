//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1157/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1157<F: Float>(t26685: F, t26692: F, t26748: F, t27808: F, t27812: F, t27904: F, t283: F, t32896: F, t95649: F, t95865: F, t96173: F, t96178: F, t96181: F, t96184: F, t96190: F, t96193: F, t96196: F, t990: F) -> (F,) {
    let t96200 = -0.37134344353515625e-4 * t32896 * t283 * t990 * t27808 + 0.74138888888888888889e-2 * t26692 * t27808 - 0.92673611111111111113e-3 * t96173 - 0.92754700520833333333e-4 * t26685 * t95649 + 0.16581944444444444444e-2 * t96178 - 0.55273148148148148147e-3 * t96181 - 0.24872916666666666666e-2 * t96184 - 0.92673611111111111112e-3 * t26748 * t27904 - 0.33163888888888888888e-2 * t96190 - 0.88437037037037037034e-2 * t96193 - 0.17687407407407407407e-1 * t96196 + 0.185671721767578125e-4 * t27812 * t95865;
    (t96200,)
}
