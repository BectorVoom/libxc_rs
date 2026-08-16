//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1294/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1294<F: Float>(t2243: F, t22722: F, t3033: F, t6194: F, t1171: F, t6198: F, t6202: F, t1185: F, t18584: F, t3070: F, t6205: F, t18427: F, t18430: F, t18433: F, t18750: F, t22230: F, t22236: F, t22262: F, t22547: F, t22550: F, t22553: F, t22556: F, t22559: F, t22688: F, t22697: F, t22699: F, t22706: F, t22721: F, t365: F, t6334: F) -> (F, F, F, F, F, F) {
    let t22724 = F::cast_from(0.48245938496077605201e2_f64) * t22722 * t2243;
    let t22726 = F::cast_from(1.0_f64) * t3033 * t6194;
    let t22727 = t1171 * t6198;
    let t22729 = F::cast_from(0.51726012919273400301e3_f64) * t22727 * t6202;
    let t22731 = F::cast_from(1.0_f64) * t18584 * t1185;
    let t22733 = F::cast_from(3.0_f64) * t6205 * t3070;
    let t22734 = t22547 + t22550 + t22553 + t22556 + t22559 - t22688 - F::cast_from(0.19751673498613801407e-1_f64) * t22697 + F::cast_from(0.10526802520742363173e2_f64) * t22699 * t6334 - F::cast_from(0.310907e-1_f64) * (t18750 - F::cast_from(0.15981777777777777777e0_f64) * t18427 + F::cast_from(0.68493333333333333333e-1_f64) * t18430 - F::cast_from(0.17123333333333333333e-1_f64) * t18433 - F::cast_from(0.53272592592592592592e-1_f64) * t22230 + t22706 - F::cast_from(0.51369999999999999999e-1_f64) * t22236 + F::cast_from(0.5137e-1_f64) * t22262) * t365 + t22721 - t22724 - t22726 - t22729 - t22731 - t22733;
    (t22724, t22726, t22729, t22731, t22733, t22734)
}
