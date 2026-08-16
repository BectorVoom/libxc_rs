//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1391/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1391(t10408: f64, t1041: f64, t10413: f64, t1616: f64, t21130: f64, t248: f64, t3062: f64, t3070: f64, t3071: f64, t42397: f64, t50181: f64, t5677: f64, t5681: f64, t5685: f64, t5867: f64, t5878: f64, t62445: f64, t62494: f64, t62559: f64, t62565: f64, t70711: f64, t70724: f64, t70766: f64, t70792: f64, t70800: f64, t70805: f64, t76589: f64) -> f64 {
    let t77724 = t70711 / 576.0_f64 + 5.0_f64 / 384.0_f64 * t1041 * t248 * t3062 * t76589 - 5.0_f64 / 10368.0_f64 * t62445 + 5.0_f64 / 2304.0_f64 * t3070 * t10408 * t5677 * t5867 + t10413 * t3071 * t5681 * t5878 / 384.0_f64 + t70724 / 576.0_f64 + 5.0_f64 / 1296.0_f64 * t3070 * t42397 * t21130 * t1616 - t62494 / 1728.0_f64 + t70766 / 1152.0_f64 + t62559 / 108.0_f64 - t62565 / 216.0_f64 + 5.0_f64 / 1728.0_f64 * t70792 - 5.0_f64 / 2304.0_f64 * t10413 * t10408 * t5878 * t5677 + t3070 * t3071 * t5685 * t5867 / 768.0_f64 - t70800 / 576.0_f64 + t70805 / 192.0_f64 + t50181 / 2592.0_f64;
    t77724
}
