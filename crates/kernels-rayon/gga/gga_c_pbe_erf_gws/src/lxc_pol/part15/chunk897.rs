//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 897/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk897(t5478: f64, t5482: f64, t5437: f64, t5443: f64, t5449: f64, t5452: f64, t7775: f64, t7779: f64, t7780: f64, t7781: f64, t7784: f64, t7788: f64, t7790: f64, t7792: f64, t7795: f64, t7797: f64, t7799: f64) -> (f64, f64, f64) {
    let t7800 = 8.0_f64 / 135.0_f64 * t5478;
    let t7801 = 8.0_f64 / 81.0_f64 * t5482;
    let t7802 = -4.0_f64 / 27.0_f64 * t5437 - t5443 + t5449 / 3.0_f64 + 0.60777777777777777777e-1_f64 * t5452 + t7775 + t7779 + t7780 - t7781 - t7784 - t7788 - t7790 - t7792 + t7795 + t7797 - t7799 + t7800 + t7801;
    (t7800, t7801, t7802)
}
