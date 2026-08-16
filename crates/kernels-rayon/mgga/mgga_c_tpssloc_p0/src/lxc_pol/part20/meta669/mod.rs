//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2515;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta669(t11243: f64, t3271: f64, t4756: f64, t1102: f64, t14758: f64, t3270: f64, t3287: f64, t51000: f64, t51004: f64, t51007: f64, t51010: f64, t51012: f64, t51014: f64, t51016: f64, t51018: f64, t51021: f64, t44938: f64, t45971: f64, t48140: f64, t45192: f64, t2403: f64, t4775: f64, t14795: f64, t699: f64, t14798: f64, t136: f64, t3297: f64, t50959: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51024, t51027, t51030, t51032) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2515(t11243, t3271, t4756, t1102, t14758, t3270, t3287, t51000, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021);
        let (t51034, t51037, t51039, t51040, t51041, t51043, t51046) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2516(t44938, t45971, t48140, t45192, t2403, t4775, t14795, t699, t14798, t136, t3297, t50959);
    (t51024, t51027, t51030, t51032, t51034, t51037, t51039, t51040, t51041, t51043, t51046)
}
