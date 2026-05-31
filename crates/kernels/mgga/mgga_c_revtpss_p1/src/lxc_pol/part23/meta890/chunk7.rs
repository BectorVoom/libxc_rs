//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2838/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2838<F: Float>(t10777: F, t10779: F, t6035: F, t61715: F, t10871: F, t4423: F, t14931: F, t23334: F, t61956: F, t10811: F, t23331: F, t10770: F, t14676: F, t14894: F, t18426: F, t18444: F, t18469: F, t18637: F, t2723: F, t2745: F, t2747: F, t40664: F, t40673: F, t40737: F, t4343: F, t4362: F, t4364: F, t4366: F, t4424: F, t6017: F, t61791: F, t76284: F, t76677: F, t76689: F, t76701: F, t76703: F, t76705: F, t837: F) -> (F, F) {
    let t76720 = t10777 * t10779 * t61715 * t6035;
    let t76726 = t10871 * t4423;
    let t76738 = t14931 * t10779 * t61956 * t23334;
    let t76740 = t10811 * t23331;
    let t76742 = t40737 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t76677 + F::cast_from(0.30011812682648815881e-2_f64) * t4362 * t4364 * t76284 * t4366 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t18426 * t18637 + F::cast_from(0.15246000842785598467e-3_f64) * t76689 - F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t4364 * t14676 * t6017 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t10770 * t18469 * t4424 - F::cast_from(0.7623000421392799234e-3_f64) * t76701 - F::cast_from(0.60023625365297631763e-2_f64) * t76703 + F::cast_from(0.25724410870841842183e-1_f64) * t2745 * t40673 * t76705 * t837 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t61791 * t6035 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t18444 * t18637 + F::cast_from(0.15246000842785598467e-3_f64) * t76720 + F::cast_from(0.51448821741683684368e-2_f64) * t14894 * t2747 * t76284 * t40664 - F::cast_from(0.38586616306262763276e-2_f64) * t14894 * t4364 * t18426 * t76726 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t2747 * t18426 * t2723 * t4343 - F::cast_from(0.30492001685571196936e-3_f64) * t76738 + F::cast_from(0.60023625365297631763e-1_f64) * t76740;
    (t76726, t76742)
}
