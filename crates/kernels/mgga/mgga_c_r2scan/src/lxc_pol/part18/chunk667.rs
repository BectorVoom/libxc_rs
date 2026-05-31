//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 667/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk667<F: Float>(t1383: F, t84: F, t4811: F, t1481: F, t382: F, t14: F, t1484: F, t31: F, t4824: F, t438: F, t4728: F, t1433: F, t1436: F, t1451: F, t1492: F, t1499: F, t1502: F, t1505: F, t377: F, t447: F, t4791: F, t4794: F, t4798: F, t4806: F, t4807: F, t4812: F, t4817: F, t4818: F, t4827: F, t4839: F, t4842: F, t4845: F, t4855: F, t4860: F, t625: F) -> (F, F, F) {
    let t4862 = F::cast_from(1.0_f64) / t1383 / t84;
    let t4863 = t4811 * t4862;
    let t4867 = F::cast_from(1.0_f64) / t1481 / t382;
    let t4868 = t14 * t4867;
    let t4870 = F::cast_from(1.0_f64) / t1484 / t31;
    let t4871 = t4824 * t4870;
    let t4872 = t4868 * t4871;
    let t4873 = F::cast_from(0.51726012919273400301e3_f64) * t4872;
    let t4874 = t4728 * t438;
    let t4877 = -F::cast_from(0.16265371950452609763e-1_f64) * t625 * t1492 * t1502 - t4791 + t4794 + t4798 + F::cast_from(0.10274e0_f64) * t625 * t377 * t1433 * t1436 - t4806 + F::cast_from(0.32530743900905219526e-1_f64) * t625 * t4807 * t1499 + F::cast_from(0.35089341735807877242e1_f64) * t1505 * t4812 - F::cast_from(0.10389515463408878255e3_f64) * t4817 * t4818 + t4827 - t4839 - t4842 + t4845 + F::cast_from(0.5848223622634646207e0_f64) * t447 * t4855 + F::cast_from(0.10254018858216406658e4_f64) * t4860 * t4863 - t4873 + F::cast_from(6.0_f64) * t1451 * t4874;
    (t4862, t4873, t4877)
}
