//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1831;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta459(t1268: f64, t1458: f64, t19451: f64, t20293: f64, t20296: f64, t20347: f64, t4028: f64, t5493: f64, t7676: f64, t19542: f64, t19576: f64, t1799: f64, t6330: f64, t15875: f64, t15877: f64, t15890: f64, t15895: f64, t19591: f64, t11982: f64, t11984: f64, t193: f64, t20077: f64, t3918: f64, t5160: f64, t5161: f64, t571: f64, t6463: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20350, t20354, t20355, t20356) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1831(t1268, t1458, t19451, t20293, t20296, t20347, t4028, t5493, t7676, t19542, t19576, t1799, t6330);
        let (t20360, t20361, t20365, t20366, t20370, t20371) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1832(t15875, t15877, t15890, t15895, t19591, t11982, t11984, t1799, t193, t20077, t20354, t20355, t20356, t3918, t5160, t5161, t571, t6463, t9457, t9476, t9484);
    (t20350, t20354, t20355, t20356, t20360, t20361, t20365, t20366, t20370, t20371)
}
