//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3133/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3133<F: Float>(t13036: F, t3597: F, t57403: F, t12772: F, t17678: F, t5340: F, t17683: F, t5331: F, t1222: F, t12809: F, t12876: F, t12910: F, t13048: F, t13055: F, t16771: F, t17461: F, t21306: F, t3720: F, t44624: F, t44649: F, t44658: F, t44661: F, t44672: F, t5308: F, t5332: F, t5405: F, t56153: F, t56224: F, t57735: F, t57737: F, t57743: F, t57746: F, t57749: F, t57759: F) -> F {
    let t57763 = t13036 * t3597 * t57403;
    let t57770 = t5340 * t12772 * t17678;
    let t57773 = t5331 * t12772 * t17683;
    let t57779 = -F::cast_from(0.64311027177104605458e-3_f64) * t21306 * t12876 + F::cast_from(0.25724410870841842183e-2_f64) * t44624 * t17461 - F::cast_from(0.17149607247227894789e-2_f64) * t57735 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t5332 * t57737 - t57743 / F::new(72.0) - t57746 / F::new(144.0) - t57749 / F::new(48.0) - t1222 * t5308 * t56224 / F::new(48.0) - t1222 * t5308 * t56153 / F::new(48.0) + F::cast_from(0.28582678745379824648e-3_f64) * t44649 - F::cast_from(0.68598428988911579154e-2_f64) * t57759 * t13048 + F::cast_from(0.68598428988911579154e-2_f64) * t57763 * t13055 + F::cast_from(0.85748036236139473944e-3_f64) * t44658 + F::cast_from(0.85748036236139473944e-3_f64) * t44661 - F::cast_from(0.42874018118069736972e-3_f64) * t44672 - F::cast_from(0.57165357490759649295e-3_f64) * t57770 + F::cast_from(0.28582678745379824648e-3_f64) * t57773 + F::cast_from(0.25724410870841842183e-2_f64) * t12910 * t3720 * t16771 * t5405;
    t57779
}
