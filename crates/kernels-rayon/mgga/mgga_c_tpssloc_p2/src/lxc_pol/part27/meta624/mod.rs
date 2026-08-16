//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2105;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta624(t12521: f64, t7467: f64, t81440: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64, t22470: f64, t4067: f64, t2332: f64, t81446: f64, t666: f64, t22473: f64, t2358: f64, t12808: f64, t6530: f64, t81438: f64, t81443: f64, t81445: f64, t109: f64, t1401: f64, t55571: f64, t7769: f64, t20173: f64, t26542: f64, t26545: f64, t12524: f64, t1458: f64, t22479: f64, t3941: f64, t4072: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86582, t86583, t86586, t86589, t86591, t86593, t86595) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2104(t12521, t7467, t81440, t1453, t81439, t26129, t81442, t22470, t4067, t2332, t81446, t666);
        let t86603 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2105(t22473, t86595, t1453, t2358, t12808, t6530, t81438, t81443, t81445, t86583, t86586, t86589, t86591, t86593);
        let (t86604, t86606, t86610, t86612, t86614, t86616, t86619, t86622) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2106(t109, t86603, t1401, t55571, t7769, t20173, t26542, t26545, t12524, t1458, t22479, t3941, t4072, t6534);
    (t86582, t86604, t86606, t86610, t86612, t86614, t86616, t86619, t86622)
}
