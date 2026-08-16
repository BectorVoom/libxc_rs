//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1515;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta506<F: Float>(t22671: F, t706: F, t750: F, t10439: F, t22688: F, t23211: F, t72: F, t757: F, t18263: F, t4305: F, t189: F, t177: F, t762: F, t23210: F, t705: F, t221: F, t23245: F, t2484: F, t2485: F, t23168: F, t40352: F, t1568: F, t6016: F, t231: F, t2782: F, t2783: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t76959, t76965, t76972, t76979, t77042, t77047) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1515::<F>(t22671, t706, t750, t10439, t22688, t23211, t72, t757, t18263, t4305, t189, t177, t762);
        let (t77054, t77127, t77131, t77159, t77171) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1516::<F>(t23210, t705, t221, t23245, t2484, t2485, t23168, t40352, t1568, t6016, t231, t2782, t2783);
    (t76959, t76965, t76972, t76979, t77042, t77047, t77054, t77127, t77131, t77159, t77171)
}
