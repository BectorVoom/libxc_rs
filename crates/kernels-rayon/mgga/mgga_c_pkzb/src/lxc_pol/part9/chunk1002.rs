//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1002/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1002(t7979: f64, t7982: f64, t6177: f64, t6256: f64, t7970: f64, t7973: f64, t7975: f64, t7986: f64, t7990: f64, t7994: f64, t7997: f64, t8000: f64) -> f64 {
    let t8059 = 0.41678e0_f64 * t7979;
    let t8060 = 0.41678e0_f64 * t7982;
    let t8066 = -0.17648625e1_f64 * t7970 + 0.6311625e0_f64 * t7973 + 0.31558125e0_f64 * t7975 - t6256 + 0.69463333333333333333e0_f64 * t6177 - t8059 - t8060 + 0.312585e0_f64 * t7986 + 0.62517e0_f64 * t7990 + 0.312585e0_f64 * t7994 + 0.264729375e1_f64 * t7997 - 0.157790625e0_f64 * t8000;
    t8066
}
