//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1138/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1138(t22739: f64, t22903: f64, t1378: f64, t22751: f64, t6892: f64, t6883: f64, t6908: f64, t2015: f64, t3911: f64, t3887: f64, t3719: f64, t6890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22904 = t22739 + t22903;
    let t22905 = t1378 * t22904;
    let t22907 = t22751 * t6892;
    let t22908 = 0.76763589786250567036e-1_f64 * t22907;
    let t22909 = t6883 * t6908;
    let t22910 = 0.38381794893125283518e-1_f64 * t22909;
    let t22912 = t2015 * t3911;
    let t22913 = t3887 * t22912;
    let t22916 = t6890 * t3719;
    (t22904, t22905, t22908, t22910, t22913, t22916)
}
