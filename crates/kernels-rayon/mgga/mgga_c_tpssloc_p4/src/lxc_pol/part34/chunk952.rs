//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 952/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk952(t11310: f64, t11365: f64, t1148: f64, t15126: f64, t15136: f64, t15207: f64, t21827: f64, t21901: f64, t21907: f64, t21939: f64, t21942: f64, t21947: f64, t21952: f64, t21956: f64, t21958: f64, t21960: f64, t21963: f64, t21975: f64, t21990: f64, t3357: f64, t3401: f64, t436: f64, t4835: f64, t6037: f64, t6069: f64, t6085: f64, t6088: f64) -> f64 {
    let t21991 = -t21901 + 0.17544670867903938621e1_f64 * t4835 * t6085 + 0.51947577317044391276e2_f64 * t15126 * t6088 - 0.10389515463408878255e3_f64 * t11365 * t21907 + 0.5848223622634646207e0_f64 * t1148 * t21939 + 0.10254018858216406658e4_f64 * t11310 * t21942 - 0.35089341735807877242e1_f64 * t15136 * t6069 + 0.35089341735807877242e1_f64 * t3401 * t21947 - 6.0_f64 * t15207 * t6037 + 6.0_f64 * t3357 * t21952 - t21956 - t21958 - t21960 + t21963 - 0.19751673498613801407e-1_f64 * t21827 - 0.310907e-1_f64 * t21975 * t436 + t21990;
    t21991
}
