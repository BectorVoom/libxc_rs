//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3161/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3161<F: Float>(t12866: F, t17661: F, t17693: F, t17694: F, t17799: F, t20934: F, t20947: F, t21173: F, t21218: F, t21306: F, t57660: F, t58899: F, t59362: F, t70032: F, t70496: F, t83018: F, t83024: F, t83034: F, t83040: F, t83047: F) -> F {
    let t83051 = -F::cast_from(0.96545937095505185473e-2_f64) * t83018 + F::cast_from(0.19055119163586549765e-3_f64) * t70032 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17661 * t21218 - F::cast_from(0.19055119163586549765e-2_f64) * t17693 * t59362 * t83024 + F::cast_from(0.42874018118069736972e-2_f64) * t17693 * t58899 * t83024 + F::cast_from(0.14291339372689912324e-2_f64) * t70496 * t20947 + F::cast_from(0.7145669686344956162e-3_f64) * t17693 * t17694 * t83034 - F::cast_from(0.17149607247227894789e-2_f64) * t17693 * t17799 * t83040 - F::cast_from(0.45732285992607719436e-2_f64) * t57660 * t20934 - F::cast_from(0.28582678745379824648e-3_f64) * t83047 + F::cast_from(0.42874018118069736972e-3_f64) * t21306 * t21173;
    t83051
}
