//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1148/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1148(t132: f64, t435: f64, t7718: f64, t6851: f64, t831: f64, t161: f64, t1639: f64, t166: f64, t7857: f64, t486: f64, t7859: f64, t13440: f64, t20784: f64, t20786: f64, t20789: f64, t20791: f64, t20792: f64, t20794: f64) -> (f64, f64, f64, f64, f64) {
    let t20796 = t132 * t435 * t7718;
    let t20797 = 2.0_f64 / 15.0_f64 * t20796;
    let t20798 = t831 * t6851;
    let t20799 = 2.0_f64 / 15.0_f64 * t20798;
    let t20803 = t161 * t166 * t1639 * t7857 / 30.0_f64;
    let t20805 = t486 * t7859 / 30.0_f64;
    let t20806 = -t20784 - t20786 - t20789 - t20791 - t20792 - t20794 + t13440 + t20797 + t20799 - t20803 - t20805;
    (t20797, t20799, t20803, t20805, t20806)
}
