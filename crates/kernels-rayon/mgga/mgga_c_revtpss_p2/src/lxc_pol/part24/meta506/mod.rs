//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1515;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta506(t22671: f64, t706: f64, t750: f64, t10439: f64, t22688: f64, t23211: f64, t72: f64, t757: f64, t18263: f64, t4305: f64, t189: f64, t177: f64, t762: f64, t23210: f64, t705: f64, t221: f64, t23245: f64, t2484: f64, t2485: f64, t23168: f64, t40352: f64, t1568: f64, t6016: f64, t231: f64, t2782: f64, t2783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76959, t76965, t76972, t76979, t77042, t77047) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1515(t22671, t706, t750, t10439, t22688, t23211, t72, t757, t18263, t4305, t189, t177, t762);
        let (t77054, t77127, t77131, t77159, t77171) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1516(t23210, t705, t221, t23245, t2484, t2485, t23168, t40352, t1568, t6016, t231, t2782, t2783);
    (t76959, t76965, t76972, t76979, t77042, t77047, t77054, t77127, t77131, t77159, t77171)
}
