//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3129/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3129<F: Float>(t11772: F, t17394: F, t3717: F, t12865: F, t17400: F, t1042: F, t12629: F, t12866: F, t12868: F, t12945: F, t12956: F, t13048: F, t1469: F, t17344: F, t17351: F, t17536: F, t17539: F, t17649: F, t17651: F, t17672: F, t17693: F, t17713: F, t17799: F, t247: F, t3719: F, t44230: F, t44561: F, t44607: F, t44616: F, t5296: F, t5297: F, t5384: F, t5391: F, t5405: F, t5407: F, t56543: F, t56907: F, t57615: F, t57621: F, t57622: F, t57631: F, t57636: F, t57641: F) -> F {
    let t57659 = t17394 * t11772;
    let t57660 = t3717 * t57659;
    let t57663 = t17400 * t12865;
    let t57667 = F::cast_from(0.63517063878621832551e-4_f64) * t57615 - F::cast_from(0.42874018118069736972e-3_f64) * t44230 * t5407 + F::cast_from(0.85748036236139473944e-3_f64) * t12956 * t17536 - F::cast_from(0.25724410870841842183e-2_f64) * t17693 * t57621 * t57622 + F::cast_from(0.85748036236139473944e-3_f64) * t17351 * t17799 * t56907 + F::cast_from(0.85748036236139473944e-3_f64) * t44561 * t17651 - t44607 + F::cast_from(0.38586616306262763275e-2_f64) * t57631 * t17713 - t57636 + F::cast_from(0.42874018118069736972e-3_f64) * t5384 * t247 * t3719 * t56543 + F::cast_from(0.12862205435420921092e-2_f64) * t57641 * t13048 - F::cast_from(0.3811023832717309953e-2_f64) * t5391 * t12945 + F::cast_from(0.85748036236139473944e-3_f64) * t17344 * t1042 * t5296 * t1469 * t12629 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17649 * t17539 * t5405 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17649 * t5297 * t17672 - F::cast_from(0.45732285992607719436e-2_f64) * t57660 * t12868 + F::cast_from(0.85748036236139473944e-3_f64) * t57663 * t12868 + F::cast_from(0.85748036236139473944e-3_f64) * t44616;
    t57667
}
