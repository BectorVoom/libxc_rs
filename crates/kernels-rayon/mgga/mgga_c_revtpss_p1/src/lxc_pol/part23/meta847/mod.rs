//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta847 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2728;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta847(t12916: f64, t20837: f64, t5331: f64, t12910: f64, t21003: f64, t12809: f64, t21029: f64, t21177: f64, t3678: f64, t17303: f64, t5327: f64, t11249: f64, t1248: f64, t1284: f64, t20849: f64, t3624: f64, t12772: f64, t17729: f64, t21036: f64, t3625: f64, t44250: f64, t6639: f64, t17423: f64, t21049: f64, t21439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70685, t70689, t70733, t70756, t70758, t70794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2728(t12916, t20837, t5331, t12910, t21003, t12809, t21029, t21177, t3678, t17303, t5327, t11249, t1248);
        let (t70800, t70806, t70809, t70811, t70819) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2729(t1284, t20849, t3624, t12772, t17729, t21036, t3625, t44250, t6639, t17423, t21049, t21439);
    (t70685, t70689, t70733, t70756, t70758, t70794, t70800, t70806, t70809, t70811, t70819)
}
