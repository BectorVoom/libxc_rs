//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1122/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1122(t10928: f64, t10930: f64, t10935: f64, t10939: f64, t10942: f64, t10946: f64, t10950: f64, t7035: f64, t7037: f64, t9159: f64, t9217: f64, t9218: f64) -> f64 {
    let t11066 = 0.31558125e0_f64 * t10928 + 0.6311625e0_f64 * t10930 - t7035 + 0.34731666666666666666e0_f64 * t7037 + 0.69463333333333333333e0_f64 * t9159 - t9217 - t9218 - 0.20839e0_f64 * t10935 + 0.62517e0_f64 * t10939 - 0.20839e0_f64 * t10942 + 0.312585e0_f64 * t10946 + 0.312585e0_f64 * t10950;
    t11066
}
