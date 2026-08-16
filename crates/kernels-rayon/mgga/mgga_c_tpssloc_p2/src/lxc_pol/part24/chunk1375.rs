//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1375/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1375(t10348: f64, t1058: f64, t1060: f64, t11065: f64, t11066: f64, t1949: f64, t23327: f64, t23346: f64, t23613: f64, t23647: f64, t23685: f64, t23686: f64, t23714: f64, t23715: f64, t2776: f64, t3010: f64, t3120: f64, t6687: f64, t6768: f64, t6784: f64, t6805: f64, t82714: f64, t82717: f64, t82730: f64, t82734: f64, t82737: f64, t82739: f64) -> f64 {
    let t82749 = 0.43864908449286038307e-1_f64 * t23346 * t23715 - 0.16449340668482264365e-1_f64 * t23327 * t23613 * t23686 - 0.43864908449286038307e-1_f64 * t82714 - 0.54831135561607547884e-2_f64 * t82717 + 0.16449340668482264365e-1_f64 * t23327 * t23613 * t23714 - 0.24674011002723396548e-1_f64 * t6687 * t3010 * t6805 - 0.82246703342411321825e-2_f64 * t6687 * t10348 * t1949 + 0.13159472534785811492e0_f64 * t23346 * t23647 - 6.0_f64 * t11065 * t82730 * t11066 + 0.82246703342411321826e-2_f64 * t82734 + 0.16449340668482264365e-1_f64 * t82737 - 0.82246703342411321826e-2_f64 * t82739 - 0.16449340668482264365e-1_f64 * t6687 * t6784 * t23685 * t2776 + 3.0_f64 * t1058 * t6768 * t3120 * t1060;
    t82749
}
