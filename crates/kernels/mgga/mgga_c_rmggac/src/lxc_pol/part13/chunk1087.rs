//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1087/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1087<F: Float>(t275: F, t9598: F, t37393: F, t570: F, t2471: F, t833: F, t40681: F, t11905: F, t1356: F, t2205: F, t2604: F, t36402: F, t36416: F, t36418: F, t37904: F, t40652: F, t40654: F, t40659: F, t40662: F, t40664: F, t40668: F, t40672: F, t40679: F, t739: F, t9332: F) -> (F, F, F) {
    let t43654 = F::new(2.0) * t275 * t9598;
    let t43655 = t37393 * t570;
    let t43658 = t2471 * t833;
    let t43677 = F::cast_from(0.66211599834018861287e-4_f64) * t40681;
    let t43678 = t43654 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t43655 - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t43658 - F::cast_from(0.11974241701863808564e0_f64) * t2604 * t9332 + F::new(2.0) * t37904 + F::cast_from(0.2553875993597870364e-4_f64) * t40652 - F::cast_from(0.79453919800822633545e-4_f64) * t40654 - F::cast_from(0.638468998399467591e-4_f64) * t40659 - F::cast_from(0.1702583995731913576e-4_f64) * t40662 + F::cast_from(0.3405167991463827152e-4_f64) * t40664 + F::cast_from(0.3405167991463827152e-4_f64) * t40668 + F::cast_from(0.1702583995731913576e-4_f64) * t40672 + F::cast_from(0.40002837092893167872e0_f64) * t36402 - F::cast_from(0.11974241701863808564e0_f64) * t11905 * t2205 + F::cast_from(0.10909864661698136692e0_f64) * t36416 - F::cast_from(0.1454648621559751559e0_f64) * t36418 - F::cast_from(0.8276449979252357661e-4_f64) * t40679 - t43677;
    (t43655, t43658, t43678)
}
