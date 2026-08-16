//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk645;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk646;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta119(t3243: f64, t3297: f64, t136: f64, t1113: f64, t3248: f64, t3252: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t3272: f64, t3280: f64, t3282: f64, t3288: f64, t3290: f64, t3294: f64, t3295: f64, t1118: f64, t1099: f64, t1097: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3298, t3299, t3301, t3302, t3304, t3305, t3307) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk645(t3243, t3297, t136, t1113, t3248, t3252, t3238, t3245, t3250, t3254, t3272, t3280, t3282, t3288, t3290, t3294, t3295);
        let (t3308, t3310, t3311) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk646(t1118, t3307, t1099, t1097);
        let (t3312, t3313) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk647(t3311, t409);
    (t3298, t3299, t3301, t3302, t3304, t3305, t3307, t3308, t3310, t3311, t3312, t3313)
}
