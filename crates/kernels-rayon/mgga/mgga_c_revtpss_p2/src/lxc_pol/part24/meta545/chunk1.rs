//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1613/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1613(t87529: f64, t87541: f64, t5962: f64, t5966: f64, t124: f64, t1544: f64, t1559: f64, t23266: f64, t2730: f64, t2745: f64, t2747: f64, t40507: f64, t40607: f64, t40611: f64, t40868: f64, t50436: f64, t50611: f64, t61677: f64, t61699: f64, t61797: f64, t61833: f64, t76279: f64, t76500: f64, t76502: f64, t76572: f64, t799: f64, t800: f64) -> (f64, f64, f64, f64) {
    let t87543 = t87529 / 2.0_f64 + t87541 / 2.0_f64;
    let t87548 = t5962 * t5962;
    let t87553 = t5966 * t5966;
    let t87562 = 0.68026775414003982664e0_f64 * t61677 + 0.27210710165601593065e0_f64 * t61699 + t2730 * t800 * t23266 * t1544 / 4.0_f64 + 0.12004725073059526352e-1_f64 * t76500 + 0.34299214494455789577e-2_f64 * t2745 * t2747 * t76279 * t1559 + 0.96037800584476210818e-1_f64 * t76502 - 0.80328230880474379775e-6_f64 * t50436 + t40507 - t799 * t800 * t124 * t87543 / 48.0_f64 + 3.0_f64 / 16.0_f64 * t2730 * t800 * t124 * t87548 + 5.0_f64 / 4.0_f64 * t40868 * t800 * t124 * t87553 + 0.15246000842785598467e-4_f64 * t61797 + 0.32528867398167352889e-3_f64 * t50611 - 0.30492001685571196936e-3_f64 * t61833 - 0.17149607247227894789e-3_f64 * t76572 + t40607 - t40611;
    (t87543, t87548, t87553, t87562)
}
