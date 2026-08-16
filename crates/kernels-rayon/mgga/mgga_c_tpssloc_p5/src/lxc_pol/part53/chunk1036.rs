//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1036/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1036(t225: f64, t33815: f64, t120350: f64, t120363: f64, t120375: f64, t113966: f64, t114000: f64, t115450: f64, t117217: f64, t120342: f64, t120344: f64, t120348: f64, t120357: f64, t120366: f64, t120369: f64, t120372: f64, t120377: f64, t120379: f64, t120381: f64, t120383: f64) -> (f64, f64) {
    let t124124 = t33815 * t225;
    let t124139 = 7.0_f64 / 576.0_f64 * t120350;
    let t124142 = 0.5383034145885385447e-3_f64 * t120363;
    let t124146 = 7.0_f64 / 144.0_f64 * t120375;
    let t124152 = -t120342 / 384.0_f64 - t120344 / 384.0_f64 - t120348 / 384.0_f64 + t124139 + 5.0_f64 / 96.0_f64 * t120357 + 0.22608743412718618877e-1_f64 * t113966 + t124142 - t117217 + 0.19378922925187387609e-1_f64 * t120366 + 0.19378922925187387609e-1_f64 * t120369 - 0.32298204875312312682e-2_f64 * t120372 + t115450 + t124146 - t120377 / 96.0_f64 - t120379 / 96.0_f64 - t120381 / 96.0_f64 + 0.13565246047631171326e0_f64 * t120383 + 0.13565246047631171326e0_f64 * t114000;
    (t124124, t124152)
}
