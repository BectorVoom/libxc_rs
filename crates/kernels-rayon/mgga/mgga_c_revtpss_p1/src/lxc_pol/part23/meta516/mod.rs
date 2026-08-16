//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2022;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta516(t1222: f64, t21169: f64, t20795: f64, t3629: f64, t3626: f64, t1261: f64, t17412: f64, t17444: f64, t17447: f64, t17453: f64, t17474: f64, t1808: f64, t21153: f64, t21157: f64, t21161: f64, t21166: f64, t3625: f64, t3647: f64, t3718: f64, t5331: f64, t6673: f64, t1234: f64, t6594: f64) -> (f64, f64, f64, f64, f64) {
        let (t21170, t21172, t21173, t21176) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2022(t1222, t21169, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t21166, t3625, t3647, t3718, t5331, t6673);
        let t21177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2023(t1234, t6594);
    (t21170, t21172, t21173, t21176, t21177)
}
