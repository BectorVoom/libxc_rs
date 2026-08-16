//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1020/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1020(t2225: f64, t522: f64, t2221: f64, t2223: f64, t2516: f64, t521: f64) -> (f64, f64, f64, f64) {
    let t3819 = 20.0_f64 * t2225 * t522;
    let t3821 = 12.0_f64 * t2221 * t522;
    let t3823 = 32.0_f64 * t2223 * t522;
    let t3824 = t521 * t2516;
    (t3819, t3821, t3823, t3824)
}
