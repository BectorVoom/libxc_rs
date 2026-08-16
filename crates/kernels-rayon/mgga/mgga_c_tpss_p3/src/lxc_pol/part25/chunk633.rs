//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 633/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk633(t2515: f64, t3749: f64, t141: f64, t3754: f64, t861: f64, t3758: f64, t2455: f64, t2499: f64, t2512: f64, t2513: f64, t3746: f64, t3751: f64, t3756: f64, t3760: f64, t3774: f64, t3782: f64, t3790: f64, t3792: f64, t3795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3797 = t2515 * t3749;
    let t3798 = t141 * t3797;
    let t3800 = t861 * t3754;
    let t3801 = t141 * t3800;
    let t3803 = t861 * t3758;
    let t3804 = t141 * t3803;
    let t3806 = -0.9494625e0_f64 * t3774 + 0.1898925e1_f64 * t3782 + t2499 + 0.99655555555555555557e-1_f64 * t2455 + 0.99655555555555555557e-1_f64 * t3746 - 0.19931111111111111111e0_f64 * t3751 + 0.59793333333333333334e0_f64 * t3756 - 0.29896666666666666667e0_f64 * t3760 + 0.15358125e0_f64 * t3790 + 0.3071625e0_f64 * t3792 + t2512 + 0.54771111111111111111e-1_f64 * t2513 + 0.54771111111111111111e-1_f64 * t3795 - 0.27385555555555555556e-1_f64 * t3798 + 0.16431333333333333333e0_f64 * t3801 - 0.82156666666666666667e-1_f64 * t3804;
    (t3797, t3798, t3800, t3801, t3803, t3804, t3806)
}
