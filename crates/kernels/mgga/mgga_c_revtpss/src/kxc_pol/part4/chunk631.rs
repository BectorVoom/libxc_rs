//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 631/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk631<F: Float>(t3059: F, t996: F, t1071: F, t994: F, t1096: F, t999: F, t1079: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F) -> (F, F, F, F, F, F) {
    let t3060 = t996 * t3059;
    let t3063 = t994 * t1071;
    let t3066 = t999 * t1096;
    let t3067 = t1079 * t3066;
    let t3070 = F::new(0.19755555555555555556e-1) * t2846;
    let t3075 = t3070 + F::new(0.9877777777777777778e-2) * t2848 - F::new(0.9877777777777777778e-2) * t2855 + F::new(0.29633333333333333334e-1) * t2860 - F::new(0.14816666666666666667e-1) * t2864;
    (t3060, t3063, t3066, t3067, t3070, t3075)
}
