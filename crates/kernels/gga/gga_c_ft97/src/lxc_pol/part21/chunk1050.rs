//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1050/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1050<F: Float>(t1786: F, t5675: F, t8326: F, t23054: F, t25920: F, t25925: F, t25930: F, t25798: F, t3626: F, t5578: F, t5611: F, t25714: F, t93324: F, t22513: F, t22572: F, t25718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t100453 = t1786 * t5675;
    let t100459 = t8326 * t5675;
    let t100477 = t23054 * t25920;
    let t100478 = 2.0 / 27.0 * t100477;
    let t100479 = t23054 * t25925;
    let t100480 = 2.0 / 27.0 * t100479;
    let t100481 = t23054 * t25930;
    let t100482 = 2.0 / 81.0 * t100481;
    let t100495 = t5578 * t3626 * t25798;
    let t100496 = t5611 * t100495;
    let t100519 = t93324 * t25714;
    let t100521 = 0.10091343167942740398e-3 * t22513 * t100519;
    let t100522 = t22572 * t25718;
    (t100453, t100459, t100477, t100478, t100479, t100480, t100481, t100482, t100495, t100496, t100519, t100521, t100522)
}
