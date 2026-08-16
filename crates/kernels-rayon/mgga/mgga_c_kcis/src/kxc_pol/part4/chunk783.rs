//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 783/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk783(t303: f64, t4789: f64, t1800: f64, t922: f64, t3202: f64, t3200: f64, t1804: f64, t3210: f64, t1121: f64, t1646: f64, t3203: f64, t1133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4790 = t303 * t4789;
    let t4792 = t1800 * t922;
    let t4793 = t3202 * t4792;
    let t4794 = t3200 * t4793;
    let t4796 = t1804 * t922;
    let t4797 = t3210 * t4796;
    let t4798 = t3200 * t4797;
    let t4800 = t1646 * t1121;
    let t4801 = t3203 * t4800;
    let t4802 = t3202 * t4801;
    let t4803 = t3200 * t4802;
    let t4805 = t1646 * t1133;
    (t4790, t4792, t4793, t4794, t4796, t4797, t4798, t4801, t4802, t4803, t4805)
}
