//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 517/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk517<F: Float>(t489: F, t490: F, t5527: F, t446: F, t476: F, t1468: F, t221: F, t209: F, t1501: F, t1508: F, t1195: F, t1467: F, t4451: F, t4460: F, t4463: F, t4465: F, t4477: F, t4505: F, t4544: F, t4556: F, t4562: F, t4570: F, t488: F, t5681: F, t5685: F, t5689: F, t5693: F, t5696: F, t5698: F, t5701: F, t5705: F, t5709: F) -> F {
    let t5716 = t489 * t490 * t5527;
    let t5720 = t476 * t446;
    let t5722 = t221 * t1468 * t5720;
    let t5725 = t209 * t446;
    let t5727 = t221 * t1501 * t5725;
    let t5730 = t1508 * t5725;
    let t5731 = t221 * t5730;
    let t5734 = t5681 - F::cast_from(0.42683754404063075712e0_f64) * t4556 + F::cast_from(0.64025631606094613569e-1_f64) * t4562 - F::cast_from(0.12805126321218922714e0_f64) * t4570 - F::cast_from(0.21341877202031537856e0_f64) * t5685 + F::cast_from(0.32927467683134372692e0_f64) * t488 * t5689 - t5693 + t5696 - F::cast_from(0.16463733841567186346e0_f64) * t5698 * t5701 + F::cast_from(0.16463733841567186347e0_f64) * t1467 * t5705 - F::cast_from(0.10975822561044790898e0_f64) * t4544 * t5709 + F::cast_from(0.12805126321218922714e0_f64) * t4451 - F::cast_from(0.85367508808126151425e0_f64) * t4463 - F::cast_from(0.38415378963656768142e0_f64) * t4465 - F::cast_from(0.54879112805223954488e-1_f64) * t488 * t5716 - t4460 + F::cast_from(0.64025631606094613569e-1_f64) * t4477 - F::cast_from(0.21951645122089581796e0_f64) * t4544 * t5722 + F::cast_from(0.10975822561044790898e0_f64) * t1195 * t5727 - F::cast_from(0.32927467683134372692e0_f64) * t4505 * t5731;
    t5734
}
