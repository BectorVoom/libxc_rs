//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1379/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1379(t29851: f64, t29853: f64, t29855: f64, t29857: f64, t29860: f64, t29862: f64, t29865: f64, t29867: f64, t29870: f64, t29873: f64, t29877: f64, t29880: f64) -> f64 {
    let t29959 = -0.3529725e1_f64 * t29851 - 0.17648625e1_f64 * t29853 - 0.157790625e0_f64 * t29855 + 0.6311625e0_f64 * t29857 + 0.6311625e0_f64 * t29860 + 0.31558125e0_f64 * t29862 + 0.10589175e2_f64 * t29865 - 0.6311625e0_f64 * t29867 + 0.34731666666666666667e0_f64 * t29870 - 0.83356e0_f64 * t29873 + 0.62517e0_f64 * t29877 - 0.41678e0_f64 * t29880;
    t29959
}
