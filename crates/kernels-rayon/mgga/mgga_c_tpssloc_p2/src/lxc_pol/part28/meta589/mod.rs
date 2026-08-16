//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1883;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta589(t25060: f64, t6547: f64, t1880: f64, t23237: f64, t25216: f64, t25192: f64, t81651: f64, t82074: f64, t6552: f64, t6555: f64, t87782: f64, t23270: f64, t25038: f64, t25191: f64, t87036: f64, t25054: f64, t23196: f64, t25224: f64, t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t82124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87804, t87822, t87835, t87861, t87866) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1883(t25060, t6547, t1880, t23237, t25216, t25192, t81651, t82074, t6552, t6555, t87782, t23270, t25038, t25191, t87036);
        let (t87873, t87893, t87898, t87901, t87904) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1884(t25054, t81651, t82074, t1880, t23196, t25224, t23030, t25205, t23164, t7479, t82133, t6552, t82124);
    (t87804, t87822, t87835, t87861, t87866, t87873, t87893, t87898, t87901, t87904)
}
