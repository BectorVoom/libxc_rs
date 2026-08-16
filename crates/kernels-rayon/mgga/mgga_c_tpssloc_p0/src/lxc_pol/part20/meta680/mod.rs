//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2565;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta680(t11478: f64, t4869: f64, t11282: f64, t1164: f64, t14854: f64, t4857: f64, t14961: f64, t3411: f64, t11311: f64, t1694: f64, t44154: f64, t11947: f64, t3637: f64, t4700: f64, t5091: f64, t51641: f64, t51669: f64, t51736: f64, t51738: f64, t51741: f64, t51744: f64, t14829: f64, t3400: f64, t4883: f64, t14960: f64, t3396: f64, t15036: f64, t11126: f64, t4879: f64, t11634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51870, t51874, t51880, t51884, t51885) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2565(t11478, t4869, t11282, t1164, t14854, t4857, t14961, t3411, t11311, t1694, t44154, t11947, t3637, t4700, t5091, t51641, t51669, t51736, t51738, t51741, t51744);
        let (t51889, t51892, t51898, t51903, t51905) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2566(t1164, t14829, t3400, t4883, t14960, t3396, t15036, t3411, t11126, t4879, t11634, t4869);
    (t51870, t51874, t51880, t51884, t51885, t51889, t51892, t51898, t51903, t51905)
}
