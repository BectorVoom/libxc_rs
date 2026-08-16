//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1298/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1298(t486: f64, t21453: f64, t1378: f64, t286: f64, t25: f64, t7087: f64, t493: f64, t1371: f64, t18431: f64, t1370: f64, t12129: f64, t12131: f64, t1368: f64, t1373: f64, t16854: f64, t16858: f64, t16866: f64, t21155: f64, t21157: f64, t21163: f64, t21167: f64, t5691: f64, t5715: f64, t5728: f64) -> f64 {
    let t495 = 0.0_f64 < t486;
    let t21455 = piecewise3(t495, t21453, -t21453);
    let t21456 = t1378 * t21455;
    let t21457 = t286 * t21456;
    let t21460 = t25 * t7087;
    let t21461 = t493 * t21460;
    let t21463 = t1371 * t18431;
    let t21464 = t1370 * t21463;
    let t21467 = t16854 - t16858 / 648.0_f64 - t16866 + t12131 / 432.0_f64 - t5691 * t5728 / 9.0_f64 + t21155 / 648.0_f64 + 11.0_f64 / 324.0_f64 * t21157 * t1373 + t5691 * t5715 / 27.0_f64 + t21163 / 864.0_f64 + t12129 + t1368 * t21167 / 48.0_f64 - t493 * t21457 / 96.0_f64 - t21461 / 288.0_f64 + t1368 * t21464 / 288.0_f64;
    t21467
}
