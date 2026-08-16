//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1133/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1133(t2785: f64, t3073: f64, t450: f64, t3053: f64, t9080: f64, t1578: f64, t1113: f64, t1141: f64, t1143: f64, t12554: f64, t12580: f64, t12590: f64, t12597: f64, t12600: f64, t12607: f64, t1581: f64, t220: f64, t3124: f64, t3126: f64, t3138: f64, t3139: f64, t4293: f64, t4303: f64, t4307: f64, t4310: f64, t4314: f64, t468: f64, t9749: f64, t9759: f64, t9764: f64, t9787: f64) -> f64 {
    let t12614 = t2785 * t3073 * t450;
    let t12618 = t9080 * t3053 * t450;
    let t12621 = t1578 * t3053;
    let t12629 = t1578 * t3073;
    let t12636 = 2.0_f64 * t1113 * t1141 * t1143 * t4293 + t1141 * t1143 * t12597 + 2.0_f64 * t1141 * t1143 * t12600 + t1141 * t1143 * t12607 + t1141 * t1143 * t12629 + t12554 * t220 * t468 + 6.0_f64 * t12580 * t1581 * t9749 - 6.0_f64 * t12590 * t1581 * t9764 - t12614 * t1581 * t3138 + t12618 * t1581 * t9787 + 2.0_f64 * t12621 * t3124 * t3126 - t12621 * t3138 * t3139 + 2.0_f64 * t1581 * t3124 * t9759 + 4.0_f64 * t3124 * t4303 * t4307 + 4.0_f64 * t3124 * t4303 * t4310 - 2.0_f64 * t3138 * t4307 * t4314 - 2.0_f64 * t3138 * t4310 * t4314;
    t12636
}
