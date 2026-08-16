//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1227/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1227(t7884: f64, t8446: f64, t1240: f64, t6226: f64, t136: f64, t550: f64, t8535: f64, t3180: f64, t6012: f64, t3174: f64, t1890: f64, t8313: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24013 = t7884 * t8446;
    let t24021 = t1240 * t6226;
    let t24026 = t136 * t550 * t8535;
    let t24131 = t6012 * t3180;
    let t24133 = t6012 * t3174;
    let t24135 = t1890 * t8313;
    (t24013, t24021, t24026, t24131, t24133, t24135)
}
