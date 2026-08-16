//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1209/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1209(t27070: f64, t28093: f64, t96395: f64, t96401: f64, t96427: f64, t1281: f64, t28250: f64, t4527: f64, t7671: f64, t1655: f64, t26654: f64, t27759: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97431 = 0.30918233506944444444e-4_f64 * t27070 * t28093;
    let t97442 = 0.10317654320987654321e-2_f64 * t96395;
    let t97449 = 0.15476481481481481481e-2_f64 * t96401;
    let t97465 = 0.23214722222222222222e-2_f64 * t96427;
    let t97494 = t28250 * t1281;
    let t97561 = 2.0_f64 * t4527 * t7671;
    let t97601 = t1655 * t26654;
    let t97606 = t27759 / 8.0_f64;
    (t97431, t97442, t97449, t97465, t97494, t97561, t97601, t97606)
}
