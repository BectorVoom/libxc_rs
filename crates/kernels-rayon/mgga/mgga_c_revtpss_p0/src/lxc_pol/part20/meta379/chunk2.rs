//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1376/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1376(t2645: f64, t775: f64, t10779: f64, t10786: f64, t14931: f64, t40583: f64, t10871: f64, t10773: f64, t10811: f64, t10489: f64, t10764: f64, t10770: f64, t10771: f64, t125: f64, t14791: f64, t14894: f64, t2646: f64, t2745: f64, t2747: f64, t2754: f64, t40446: f64, t40600: f64, t40607: f64, t40611: f64, t40625: f64, t40630: f64, t40638: f64, t40639: f64, t40643: f64, t40645: f64, t40654: f64, t837: f64) -> (f64, f64) {
    let t40655 = t775 * t2645;
    let t40662 = t14931 * t10779 * t40583 * t10786;
    let t40664 = t10871 * t775;
    let t40669 = t10811 * t10773;
    let t40671 = 0.6098400337114239387e-4_f64 * t40600 + t40607 - t40611 + 0.34299214494455789577e-2_f64 * t2745 * t2747 * t125 * t10489 * t837 + 0.51448821741683684366e-2_f64 * t2745 * t2747 * t10764 * t2646 - 0.25724410870841842184e-1_f64 * t2745 * t10770 * t10771 * t2754 + 0.18071592998981862717e-5_f64 * t40625 + 0.2168591159877823526e-3_f64 * t40630 - t40638 + 0.11560105625909173524e-1_f64 * t40639 + 0.11433071498151929859e-3_f64 * t40643 - 0.18292914397043087775e-2_f64 * t40645 + t40654 + 0.10289764348336736873e-1_f64 * t2745 * t14791 * t837 * t40655 - 0.12196800674228478774e-2_f64 * t40662 + 0.20579528696673473747e-1_f64 * t14894 * t2747 * t40446 * t40664 + 0.24009450146119052704e0_f64 * t40669;
    (t40655, t40671)
}
