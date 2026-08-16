//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 518/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk518(t489: f64, t490: f64, t5527: f64, t446: f64, t476: f64, t1468: f64, t221: f64, t209: f64, t1501: f64, t1508: f64, t1195: f64, t1467: f64, t4451: f64, t4460: f64, t4463: f64, t4465: f64, t4477: f64, t4505: f64, t4544: f64, t4556: f64, t4562: f64, t4570: f64, t488: f64, t5681: f64, t5685: f64, t5689: f64, t5693: f64, t5696: f64, t5698: f64, t5701: f64, t5705: f64, t5709: f64) -> f64 {
    let t5716 = t489 * t490 * t5527;
    let t5720 = t476 * t446;
    let t5722 = t221 * t1468 * t5720;
    let t5725 = t209 * t446;
    let t5727 = t221 * t1501 * t5725;
    let t5730 = t1508 * t5725;
    let t5731 = t221 * t5730;
    let t5734 = t5681 - 0.42683754404063075712e0_f64 * t4556 + 0.64025631606094613569e-1_f64 * t4562 - 0.12805126321218922714e0_f64 * t4570 - 0.21341877202031537856e0_f64 * t5685 + 0.32927467683134372692e0_f64 * t488 * t5689 - t5693 + t5696 - 0.16463733841567186346e0_f64 * t5698 * t5701 + 0.16463733841567186347e0_f64 * t1467 * t5705 - 0.10975822561044790898e0_f64 * t4544 * t5709 + 0.12805126321218922714e0_f64 * t4451 - 0.85367508808126151425e0_f64 * t4463 - 0.38415378963656768142e0_f64 * t4465 - 0.54879112805223954488e-1_f64 * t488 * t5716 - t4460 + 0.64025631606094613569e-1_f64 * t4477 - 0.21951645122089581796e0_f64 * t4544 * t5722 + 0.10975822561044790898e0_f64 * t1195 * t5727 - 0.32927467683134372692e0_f64 * t4505 * t5731;
    t5734
}
