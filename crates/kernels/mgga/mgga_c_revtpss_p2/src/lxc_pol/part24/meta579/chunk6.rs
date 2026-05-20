//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1791/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1791<F: Float>(t1774: F, t471: F, t1042: F, t1261: F, t12866: F, t1715: F, t17344: F, t17694: F, t1797: F, t20820: F, t24652: F, t24655: F, t24808: F, t3718: F, t3720: F, t5268: F, t5373: F, t5381: F, t6625: F, t82725: F, t82799: F, t83607: F, t83992: F, t83994: F, t88916: F, t90885: F) -> F {
    let t91338 = t471 * t1774;
    let t91352 = F::cast_from(0.34299214494455789578e-2_f64) * t17344 * t1042 * t82799 * t1715 - F::cast_from(0.34299214494455789578e-2_f64) * t5381 * t24808 - F::cast_from(0.11433071498151929859e-2_f64) * t1261 * t1042 * t5268 * t88916 + F::cast_from(0.85748036236139473944e-3_f64) * t83607 * t1797 + F::cast_from(0.12862205435420921092e-2_f64) * t20820 * t6625 - F::cast_from(0.85748036236139473944e-3_f64) * t3718 * t3720 * t82725 * t91338 - F::new(4.0) / F::new(81.0) * t83992 + t83994 / F::new(27.0) - F::new(4.0) / F::new(27.0) * t5373 * t24655 + F::new(2.0) / F::new(9.0) * t5373 * t24652 - F::cast_from(0.28582678745379824648e-2_f64) * t12866 * t17694 * t90885;
    t91352
}
