//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1269/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1269<F: Float>(t119697: F, t119592: F, t39749: F, t446: F, t119562: F, t23657: F, t23671: F, t23892: F, t105425: F, t16671: F, t1901: F, t105429: F, t16675: F, t23652: F, t4454: F, t5899: F, t9049: F) -> (F, F, F, F, F, F) {
    let t119698 = t119697 / 12.0;
    let t119700 = t446 * t39749 * t119592;
    let t119704 = t23657 * t23671 * t23892 * t119562;
    let t119707 = t1901 * t105425 * t16671;
    let t119710 = t1901 * t105429 * t16675;
    let t119714 = t5899 * t9049 * t23652 * t4454;
    (t119698, t119700, t119704, t119707, t119710, t119714)
}
