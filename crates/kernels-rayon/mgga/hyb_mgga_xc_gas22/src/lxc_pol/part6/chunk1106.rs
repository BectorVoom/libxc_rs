//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1106/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1106(t132: f64, t10850: f64, t338: f64, t4222: f64, t930: f64, t10325: f64, t2598: f64, t4323: f64, t3605: f64, t3604: f64, t9057: f64, t4310: f64, t6992: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t10851 = t10850 * t338;
    let t10852 = t4222 * t930;
    let t10853 = piecewise3(t133, 0.0_f64, t10325);
    let t10864 = t2598 * t4323;
    let t10865 = t10864 * t3605;
    let t10868 = t3604 * t9057;
    let t10871 = t6992 * t4310;
    (t10851, t10852, t10853, t10864, t10865, t10868, t10871)
}
