//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1176;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta281(t246: f64, t9645: f64, t232: f64, t2379: f64, t2628: f64, t835: f64, t812: f64, t2635: f64, t2690: f64, t815: f64, t831: f64, t2617: f64, t2638: f64, t2639: f64, t2681: f64, t116: f64, t126: f64, t136: f64, t16: f64, t2386: f64, t625: f64, t2385: f64, t686: f64, t781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9646, t9647, t9668, t9671, t9672, t9674) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1176(t246, t9645, t232, t2379, t2628, t835, t812, t2635, t2690, t815, t831, t2617, t2638);
        let (t9675, t9679, t9689, t9691, t9692, t9694) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1177(t831, t9674, t2639, t2681, t116, t126, t136, t16, t2386, t625, t2385, t686, t781);
    (t9646, t9647, t9668, t9671, t9672, t9674, t9675, t9679, t9689, t9691, t9692, t9694)
}
