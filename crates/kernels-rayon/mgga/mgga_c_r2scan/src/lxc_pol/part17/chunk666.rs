//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 666/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk666(t1383: f64, t84: f64, t4811: f64, t1481: f64, t382: f64, t14: f64, t1484: f64, t31: f64, t4824: f64, t438: f64, t4728: f64, t1433: f64, t1436: f64, t1451: f64, t1492: f64, t1499: f64, t1502: f64, t1505: f64, t377: f64, t447: f64, t4791: f64, t4794: f64, t4798: f64, t4806: f64, t4807: f64, t4812: f64, t4817: f64, t4818: f64, t4827: f64, t4839: f64, t4842: f64, t4845: f64, t4855: f64, t4860: f64, t625: f64) -> (f64, f64, f64) {
    let t4862 = 1.0_f64 / t1383 / t84;
    let t4863 = t4811 * t4862;
    let t4867 = 1.0_f64 / t1481 / t382;
    let t4868 = t14 * t4867;
    let t4870 = 1.0_f64 / t1484 / t31;
    let t4871 = t4824 * t4870;
    let t4872 = t4868 * t4871;
    let t4873 = 0.51726012919273400301e3_f64 * t4872;
    let t4874 = t4728 * t438;
    let t4877 = -0.16265371950452609763e-1_f64 * t625 * t1492 * t1502 - t4791 + t4794 + t4798 + 0.10274e0_f64 * t625 * t377 * t1433 * t1436 - t4806 + 0.32530743900905219526e-1_f64 * t625 * t4807 * t1499 + 0.35089341735807877242e1_f64 * t1505 * t4812 - 0.10389515463408878255e3_f64 * t4817 * t4818 + t4827 - t4839 - t4842 + t4845 + 0.5848223622634646207e0_f64 * t447 * t4855 + 0.10254018858216406658e4_f64 * t4860 * t4863 - t4873 + 6.0_f64 * t1451 * t4874;
    (t4862, t4873, t4877)
}
