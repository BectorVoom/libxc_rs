//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1294/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1294(t2243: f64, t22722: f64, t3033: f64, t6194: f64, t1171: f64, t6198: f64, t6202: f64, t1185: f64, t18584: f64, t3070: f64, t6205: f64, t18427: f64, t18430: f64, t18433: f64, t18750: f64, t22230: f64, t22236: f64, t22262: f64, t22547: f64, t22550: f64, t22553: f64, t22556: f64, t22559: f64, t22688: f64, t22697: f64, t22699: f64, t22706: f64, t22721: f64, t365: f64, t6334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22724 = 0.48245938496077605201e2_f64 * t22722 * t2243;
    let t22726 = 1.0_f64 * t3033 * t6194;
    let t22727 = t1171 * t6198;
    let t22729 = 0.51726012919273400301e3_f64 * t22727 * t6202;
    let t22731 = 1.0_f64 * t18584 * t1185;
    let t22733 = 3.0_f64 * t6205 * t3070;
    let t22734 = t22547 + t22550 + t22553 + t22556 + t22559 - t22688 - 0.19751673498613801407e-1_f64 * t22697 + 0.10526802520742363173e2_f64 * t22699 * t6334 - 0.310907e-1_f64 * (t18750 - 0.15981777777777777777e0_f64 * t18427 + 0.68493333333333333333e-1_f64 * t18430 - 0.17123333333333333333e-1_f64 * t18433 - 0.53272592592592592592e-1_f64 * t22230 + t22706 - 0.51369999999999999999e-1_f64 * t22236 + 0.5137e-1_f64 * t22262) * t365 + t22721 - t22724 - t22726 - t22729 - t22731 - t22733;
    (t22724, t22726, t22729, t22731, t22733, t22734)
}
