//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3161/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3161(t12866: f64, t17661: f64, t17693: f64, t17694: f64, t17799: f64, t20934: f64, t20947: f64, t21173: f64, t21218: f64, t21306: f64, t57660: f64, t58899: f64, t59362: f64, t70032: f64, t70496: f64, t83018: f64, t83024: f64, t83034: f64, t83040: f64, t83047: f64) -> f64 {
    let t83051 = -0.96545937095505185473e-2_f64 * t83018 + 0.19055119163586549765e-3_f64 * t70032 + 0.42874018118069736972e-3_f64 * t12866 * t17661 * t21218 - 0.19055119163586549765e-2_f64 * t17693 * t59362 * t83024 + 0.42874018118069736972e-2_f64 * t17693 * t58899 * t83024 + 0.14291339372689912324e-2_f64 * t70496 * t20947 + 0.7145669686344956162e-3_f64 * t17693 * t17694 * t83034 - 0.17149607247227894789e-2_f64 * t17693 * t17799 * t83040 - 0.45732285992607719436e-2_f64 * t57660 * t20934 - 0.28582678745379824648e-3_f64 * t83047 + 0.42874018118069736972e-3_f64 * t21306 * t21173;
    t83051
}
