//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 835/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk835(t214: f64, t2151: f64, t215: f64, t2986: f64, t136: f64, t1815: f64, t686: f64) -> (f64, f64, f64, f64) {
    let t6457 = t2151 * t214;
    let t6466 = t2986 * t215;
    let t6468 = 5.0_f64 / 288.0_f64 * t136 * t6466;
    let t6469 = t1815 * t686;
    (t6457, t6466, t6468, t6469)
}
