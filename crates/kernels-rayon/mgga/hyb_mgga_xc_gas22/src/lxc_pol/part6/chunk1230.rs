//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1230/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1230(t1890: f64, t8317: f64, t23340: f64, t3184: f64, t1270: f64, t6359: f64, t180: f64, t8354: f64, t2004: f64, t3279: f64, t136: f64, t550: f64, t8440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24218 = t1890 * t8317;
    let t24220 = t23340 * t3184;
    let t24320 = t6359 * t1270;
    let t24354 = t180 * t8354;
    let t24426 = t2004 * t3279;
    let t24439 = t136 * t550 * t8440;
    (t24218, t24220, t24320, t24354, t24426, t24439)
}
