//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 878/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk878(t30697: f64, t30704: f64, t30721: f64, t30701: f64, t30707: f64, t30710: f64, t30717: f64, t30723: f64, t218: f64, t31374: f64, t31382: f64, t814: f64, t8728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31976 = 0.22608743412718618877e-1_f64 * t30697;
    let t31978 = 0.5383034145885385447e-3_f64 * t30704;
    let t31982 = 7.0_f64 / 576.0_f64 * t30721;
    let t31984 = -t31976 - 0.19378922925187387609e-1_f64 * t30701 - t31978 - 0.32298204875312312682e-2_f64 * t30707 + t30710 / 384.0_f64 - t30717 / 384.0_f64 - t31982 - t30723 / 96.0_f64;
    let t31985 = t218 * t31984;
    let t31987 = 0.76763589786250567037e-1_f64 * t31374;
    let t31989 = 0.16449340668482264365e-1_f64 * t31382;
    let t31993 = t814 * t8728;
    (t31976, t31978, t31982, t31984, t31985, t31987, t31989, t31993)
}
