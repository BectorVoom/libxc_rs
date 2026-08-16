//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta484(t1285: f64, t70994: f64, t1121: f64, t6587: f64, t17395: f64, t17400: f64, t20809: f64, t372: f64, t3655: f64, t6598: f64, t6602: f64, t5436: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t70995, t71029, t71081, t71112, t71187, t71192, t71275) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1476(t1285, t70994, t1121, t6587, t17395, t17400, t20809, t372, t3655, t6598, t6602, t5436);
    (t70995, t71029, t71081, t71112, t71187, t71192, t71275)
}
