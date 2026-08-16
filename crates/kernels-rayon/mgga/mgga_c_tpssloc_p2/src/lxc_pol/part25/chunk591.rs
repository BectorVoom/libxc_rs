//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 591/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk591(t2586: f64, t3749: f64, t1315: f64, t3725: f64, t3727: f64, t3731: f64, t3733: f64, t3736: f64, t3742: f64, t3745: f64) -> f64 {
    let t3751 = 0.83333333333333333332e-3_f64 * t2586 * t3749;
    let t3752 = t3725 + 0.77777777777777777775e-2_f64 * t3727 + t3731 + 0.49999999999999999998e-2_f64 * t3733 * t3736 + 0.16666666666666666666e-2_f64 * t3742 - 0.16666666666666666666e-2_f64 * t1315 * t3745 - t3751;
    t3752
}
