//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1194/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1194(t1025: f64, t2630: f64, t7255: f64, t7485: f64, t7497: f64, t1112: f64, t7345: f64, t2662: f64, t2676: f64, t2640: f64, t7491: f64, t7554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22115 = 0.38527786510141256862e1_f64 * t2630 * t1025 * t7255;
    let t22116 = t7497 * t7485;
    let t22120 = 0.67471172535210825684e-1_f64 * t2630 * t7345 * t1112;
    let t22123 = 0.43374325201206959368e-1_f64 * t2630 * t2662 * t2676;
    let t22126 = 0.12842595503380418954e1_f64 * t2630 * t2662 * t2640;
    let t22127 = t7497 * t7491;
    let t22131 = 0.21687162600603479684e-1_f64 * t2630 * t1025 * t7554;
    (t22115, t22116, t22120, t22123, t22126, t22127, t22131)
}
