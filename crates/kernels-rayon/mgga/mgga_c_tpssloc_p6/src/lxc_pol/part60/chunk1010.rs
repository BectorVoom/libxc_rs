//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1010/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1010(t1888: f64, t23270: f64, t26728: f64, t5636: f64, t121401: f64, t1880: f64, t7488: f64, t101551: f64, t113038: f64, t113045: f64, t121749: f64, t121753: f64, t126518: f64, t126520: f64, t126521: f64, t1492: f64, t17052: f64, t17090: f64, t17092: f64, t25168: f64, t259: f64, t2718: f64, t28306: f64, t29091: f64, t33395: f64, t5657: f64, t6627: f64, t7516: f64, t855: f64, t8553: f64, t8562: f64, t8563: f64) -> f64 {
    let t128049 = t1888 * t23270 * t26728 * t5636;
    let t128070 = t1880 * t121401 * t7488;
    let t128072 = -t17090 * t8563 + t113038 + 2.0_f64 * t1492 * t33395 * t259 - 0.49348022005446793095e-1_f64 * t128049 - 12.0_f64 * t25168 * t26728 * t28306 - t113045 - 12.0_f64 * t25168 * t101551 * t7516 + t126518 + 2.0_f64 * t17052 * t8553 + 4.0_f64 * t17092 * t8553 - 0.82246703342411321824e-2_f64 * t121749 + 0.82246703342411321824e-2_f64 * t121753 + 2.0_f64 * t855 * t2718 * t8562 * t5657 - 6.0_f64 * t6627 * t29091 - t126520 - 0.16449340668482264365e-1_f64 * t128070 + t126521;
    t128072
}
