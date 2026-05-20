//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1376/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1376<F: Float>(t2645: F, t775: F, t10779: F, t10786: F, t14931: F, t40583: F, t10871: F, t10773: F, t10811: F, t10489: F, t10764: F, t10770: F, t10771: F, t125: F, t14791: F, t14894: F, t2646: F, t2745: F, t2747: F, t2754: F, t40446: F, t40600: F, t40607: F, t40611: F, t40625: F, t40630: F, t40638: F, t40639: F, t40643: F, t40645: F, t40654: F, t837: F) -> (F, F) {
    let t40655 = t775 * t2645;
    let t40662 = t14931 * t10779 * t40583 * t10786;
    let t40664 = t10871 * t775;
    let t40669 = t10811 * t10773;
    let t40671 = F::cast_from(0.6098400337114239387e-4_f64) * t40600 + t40607 - t40611 + F::cast_from(0.34299214494455789577e-2_f64) * t2745 * t2747 * t125 * t10489 * t837 + F::cast_from(0.51448821741683684366e-2_f64) * t2745 * t2747 * t10764 * t2646 - F::cast_from(0.25724410870841842184e-1_f64) * t2745 * t10770 * t10771 * t2754 + F::cast_from(0.18071592998981862717e-5_f64) * t40625 + F::cast_from(0.2168591159877823526e-3_f64) * t40630 - t40638 + F::cast_from(0.11560105625909173524e-1_f64) * t40639 + F::cast_from(0.11433071498151929859e-3_f64) * t40643 - F::cast_from(0.18292914397043087775e-2_f64) * t40645 + t40654 + F::cast_from(0.10289764348336736873e-1_f64) * t2745 * t14791 * t837 * t40655 - F::cast_from(0.12196800674228478774e-2_f64) * t40662 + F::cast_from(0.20579528696673473747e-1_f64) * t14894 * t2747 * t40446 * t40664 + F::cast_from(0.24009450146119052704e0_f64) * t40669;
    (t40655, t40671)
}
