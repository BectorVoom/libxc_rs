//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1791/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1791(t1774: f64, t471: f64, t1042: f64, t1261: f64, t12866: f64, t1715: f64, t17344: f64, t17694: f64, t1797: f64, t20820: f64, t24652: f64, t24655: f64, t24808: f64, t3718: f64, t3720: f64, t5268: f64, t5373: f64, t5381: f64, t6625: f64, t82725: f64, t82799: f64, t83607: f64, t83992: f64, t83994: f64, t88916: f64, t90885: f64) -> f64 {
    let t91338 = t471 * t1774;
    let t91352 = 0.34299214494455789578e-2_f64 * t17344 * t1042 * t82799 * t1715 - 0.34299214494455789578e-2_f64 * t5381 * t24808 - 0.11433071498151929859e-2_f64 * t1261 * t1042 * t5268 * t88916 + 0.85748036236139473944e-3_f64 * t83607 * t1797 + 0.12862205435420921092e-2_f64 * t20820 * t6625 - 0.85748036236139473944e-3_f64 * t3718 * t3720 * t82725 * t91338 - 4.0_f64 / 81.0_f64 * t83992 + t83994 / 27.0_f64 - 4.0_f64 / 27.0_f64 * t5373 * t24655 + 2.0_f64 / 9.0_f64 * t5373 * t24652 - 0.28582678745379824648e-2_f64 * t12866 * t17694 * t90885;
    t91352
}
