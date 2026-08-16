//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 637/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk637(t1441: f64, t895: f64, t1449: f64, t903: f64, t2455: f64, t2513: f64, t2601: f64, t2608: f64, t3746: f64, t3751: f64, t3756: f64, t3760: f64, t3774: f64, t3782: f64, t3790: f64, t3792: f64, t3795: f64, t3798: f64, t3801: f64, t3804: f64) -> (f64, f64, f64) {
    let t3860 = t1441 * t895;
    let t3865 = t1449 * t903;
    let t3882 = -0.1294625e1_f64 * t3774 + 0.258925e1_f64 * t3782 + t2601 + 0.10064166666666666667e0_f64 * t2455 + 0.10064166666666666667e0_f64 * t3746 - 0.20128333333333333333e0_f64 * t3751 + 0.60385e0_f64 * t3756 - 0.301925e0_f64 * t3760 + 0.82524375e-1_f64 * t3790 + 0.16504875e0_f64 * t3792 + t2608 + 0.5519e-1_f64 * t2513 + 0.5519e-1_f64 * t3795 - 0.27595e-1_f64 * t3798 + 0.16557e0_f64 * t3801 - 0.82785e-1_f64 * t3804;
    (t3860, t3865, t3882)
}
