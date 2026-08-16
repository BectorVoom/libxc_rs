//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 904/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk904(t2: f64, t37406: f64, t37357: f64, t3139: f64, t466: f64, t1781: f64, t37362: f64, t1775: f64, t8308: f64, t8314: f64, t1791: f64, t37352: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38549 = t2 * t37406;
    let t38550 = t38549 * t37357;
    let t38554 = t3139 * t466;
    let t38556 = t1781 * t37362;
    let t38560 = t1775 * t8308;
    let t38562 = t8314 * t37357;
    let t38566 = t1791 * t37362;
    let t38570 = t37352 * t82;
    (t38550, t38554, t38556, t38560, t38562, t38566, t38570)
}
