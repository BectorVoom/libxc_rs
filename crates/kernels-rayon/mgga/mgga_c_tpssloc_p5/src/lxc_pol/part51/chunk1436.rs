//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1436/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1436(t26189: f64, t31611: f64, t6888: f64, t115352: f64, t22892: f64, t7691: f64, t12020: f64, t8636: f64, t115586: f64, t120577: f64, t120579: f64, t120590: f64, t120591: f64, t1375: f64, t1385: f64, t24082: f64, t26224: f64, t26990: f64, t27009: f64, t31564: f64, t33293: f64, t3887: f64, t5215: f64, t5325: f64, t5353: f64, t6993: f64, t7728: f64, t7729: f64, t93818: f64) -> f64 {
    let t122328 = t6888 * t31611 * t26189;
    let t122331 = t22892 * t115352 * t7691;
    let t122335 = t12020 * t8636;
    let t122349 = -6.0_f64 * t26224 * t93818 * t7728 + t120577 + 2.0_f64 * t1375 * t3887 * t8636 * t5353 - 0.16449340668482264365e-1_f64 * t122328 + 0.82246703342411321825e-2_f64 * t122331 - 6.0_f64 * t120591 * t26990 + t120579 - 6.0_f64 * t26224 * t122335 * t5325 + 2.0_f64 * t1375 * t3887 * t33293 * t1385 + 2.0_f64 * t5215 * t31564 - t120590 + 2.0_f64 * t24082 * t7729 - 0.82246703342411321824e-2_f64 * t115586 - t27009 * t6993;
    t122349
}
