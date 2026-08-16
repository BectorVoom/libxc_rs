//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2038/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2038(t87535: f64, t23185: f64, t4283: f64, t81914: f64, t25300: f64, t81591: f64, t25303: f64, t6579: f64, t23110: f64, t4292: f64, t25288: f64, t234: f64, t4265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87536 = 0.38381794893125283518e-1_f64 * t87535;
    let t87544 = t23185 * t81914 * t4283;
    let t87545 = 0.16449340668482264365e-1_f64 * t87544;
    let t87546 = t81591 * t25300;
    let t87547 = 0.76763589786250567036e-1_f64 * t87546;
    let t87565 = t6579 * t25303;
    let t87566 = 0.76763589786250567036e-1_f64 * t87565;
    let t87581 = t23185 * t23110 * t4292;
    let t87582 = 0.82246703342411321824e-2_f64 * t87581;
    let t87583 = t81591 * t25288;
    let t87584 = 0.76763589786250567036e-1_f64 * t87583;
    let t87586 = t234 * t4265;
    (t87536, t87545, t87547, t87566, t87582, t87584, t87586)
}
