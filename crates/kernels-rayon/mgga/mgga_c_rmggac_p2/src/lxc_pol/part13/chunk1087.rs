//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1087/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1087(t275: f64, t9598: f64, t37393: f64, t570: f64, t2471: f64, t833: f64, t40681: f64, t11905: f64, t1356: f64, t2205: f64, t2604: f64, t36402: f64, t36416: f64, t36418: f64, t37904: f64, t40652: f64, t40654: f64, t40659: f64, t40662: f64, t40664: f64, t40668: f64, t40672: f64, t40679: f64, t739: f64, t9332: f64) -> (f64, f64, f64) {
    let t43654 = 2.0_f64 * t275 * t9598;
    let t43655 = t37393 * t570;
    let t43658 = t2471 * t833;
    let t43677 = 0.66211599834018861287e-4_f64 * t40681;
    let t43678 = t43654 + 0.39914139006212695214e-1_f64 * t1356 * t43655 - 0.59871208509319042821e-1_f64 * t739 * t43658 - 0.11974241701863808564e0_f64 * t2604 * t9332 + 2.0_f64 * t37904 + 0.2553875993597870364e-4_f64 * t40652 - 0.79453919800822633545e-4_f64 * t40654 - 0.638468998399467591e-4_f64 * t40659 - 0.1702583995731913576e-4_f64 * t40662 + 0.3405167991463827152e-4_f64 * t40664 + 0.3405167991463827152e-4_f64 * t40668 + 0.1702583995731913576e-4_f64 * t40672 + 0.40002837092893167872e0_f64 * t36402 - 0.11974241701863808564e0_f64 * t11905 * t2205 + 0.10909864661698136692e0_f64 * t36416 - 0.1454648621559751559e0_f64 * t36418 - 0.8276449979252357661e-4_f64 * t40679 - t43677;
    (t43655, t43658, t43678)
}
