//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 515/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk515(t209: f64, t469: f64, t5666: f64, t6: f64, t219: f64, t4467: f64, t1144: f64, t1516: f64, t4462: f64, t612: f64, t1195: f64, t1467: f64, t1500: f64, t4505: f64, t4560: f64, t467: f64, t488: f64, t5571: f64, t5574: f64, t5579: f64, t5585: f64, t5587: f64, t5592: f64, t5597: f64, t5602: f64, t5607: f64, t5611: f64, t5616: f64, t5621: f64, t5625: f64, t5630: f64, t5633: f64, t5636: f64) -> f64 {
    let t5669 = t469 * t6 * t5666 * t209;
    let t5672 = t4467 * t219;
    let t5674 = t5672 * t1516 * t1144;
    let t5677 = t4462 * t612;
    let t5679 = -t5571 + 0.54879112805223954488e-1_f64 * t1195 * t5574 - 0.27439556402611977244e-1_f64 * t1500 * t5579 - t5585 - 0.16463733841567186346e0_f64 * t4505 * t5587 + 0.10975822561044790898e0_f64 * t1195 * t5592 + 0.54879112805223954488e-1_f64 * t1195 * t5597 - 0.27439556402611977244e-1_f64 * t1500 * t5602 + 0.10975822561044790898e0_f64 * t1195 * t5607 + 0.54879112805223954488e-1_f64 * t1195 * t5611 - 0.54879112805223954488e-1_f64 * t1500 * t5616 + 0.10975822561044790898e0_f64 * t1467 * t5621 + 0.54879112805223954488e-1_f64 * t1467 * t5625 - 0.25610252642437845428e0_f64 * t4560 + 0.16463733841567186346e0_f64 * t488 * t5630 - 0.76830757927313536283e0_f64 * t5633 + t5636 - 0.27439556402611977244e-1_f64 * t467 * t5669 - 0.65854935366268745384e0_f64 * t488 * t5674 - 0.42683754404063075713e0_f64 * t5677;
    t5679
}
