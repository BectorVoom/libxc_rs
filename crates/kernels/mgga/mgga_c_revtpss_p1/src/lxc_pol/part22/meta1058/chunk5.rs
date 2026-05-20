//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3758/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3758<F: Float>(t17763: F, t5378: F, t3568: F, t6587: F, t12800: F, t12866: F, t17344: F, t17351: F, t17354: F, t17401: F, t17514: F, t17724: F, t1808: F, t21272: F, t247: F, t3620: F, t3719: F, t58863: F, t59173: F, t59176: F, t59179: F, t59182: F, t59185: F, t6673: F, t71300: F) -> (F, F) {
    let t71598 = t17763 * t5378;
    let t71606 = t6587 * t3568;
    let t71624 = -F::cast_from(0.3811023832717309953e-3_f64) * t71598 + F::cast_from(0.23818898954483187207e-3_f64) * t12800 * t6673 + F::cast_from(0.80454947579587654563e-2_f64) * t21272 * t3620 + F::cast_from(0.15244095330869239812e-2_f64) * t58863 * t1808 - F::cast_from(0.12862205435420921092e-2_f64) * t17344 * t247 * t3719 * t71606 - F::cast_from(0.28582678745379824648e-3_f64) * t59173 - F::cast_from(0.57165357490759649296e-3_f64) * t59176 + F::cast_from(0.28582678745379824648e-3_f64) * t59179 + F::cast_from(0.7622047665434619906e-3_f64) * t59182 - F::cast_from(0.85748036236139473944e-3_f64) * t17401 * t17724 + F::cast_from(0.19055119163586549765e-3_f64) * t59185 + F::cast_from(0.28582678745379824648e-3_f64) * t17351 * t71300 * t17354 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t71300 * t17514;
    (t71606, t71624)
}
