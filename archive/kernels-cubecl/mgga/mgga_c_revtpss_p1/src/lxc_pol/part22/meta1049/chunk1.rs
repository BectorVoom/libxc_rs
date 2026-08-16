//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3687/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3687<F: Float>(t1230: F, t21271: F, t1266: F, t12800: F, t17763: F, t1808: F, t21242: F, t21272: F, t3640: F, t3644: F, t5397: F, t57187: F, t6683: F, t69698: F, t69700: F, t69710: F, t69719: F, t69721: F) -> F {
    let t69723 = t1230 * t21271;
    let t69728 = -F::cast_from(0.3811023832717309953e-3_f64) * t69698 - F::cast_from(0.95275595817932748827e-4_f64) * t69700 - F::cast_from(0.96545937095505185476e-2_f64) * t21272 * t3644 - F::cast_from(0.28582678745379824648e-3_f64) * t12800 * t6683 - F::cast_from(0.28582678745379824648e-3_f64) * t57187 * t1808 - F::cast_from(0.57165357490759649296e-3_f64) * t17763 * t5397 + F::cast_from(0.30488190661738479624e-2_f64) * t69710 * t1266 + F::cast_from(0.15244095330869239812e-2_f64) * t21242 * t3640 + F::cast_from(0.30488190661738479624e-2_f64) * t21242 * t3644 + F::cast_from(0.57165357490759649296e-3_f64) * t69719 - F::cast_from(0.60976381323476959248e-2_f64) * t69721 - F::cast_from(0.96545937095505185476e-2_f64) * t69723 * t1266 - F::cast_from(0.48272968547752592738e-2_f64) * t21272 * t3640;
    t69728
}
