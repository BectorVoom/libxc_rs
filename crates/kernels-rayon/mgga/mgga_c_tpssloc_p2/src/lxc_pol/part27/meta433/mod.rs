//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta433(t1354: f64, t22765: f64, t3858: f64, t6945: f64, t1339: f64, t3851: f64, t6936: f64, t3856: f64, t3788: f64, t3793: f64, t6604: f64, t6919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22766, t22767, t22768, t22770, t22771, t22773, t22774, t22776, t22777, t22779) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1754(t1354, t22765, t3858, t6945, t1339, t3851, t6936, t3856, t3788, t3793, t6604, t6919);
    (t22766, t22767, t22768, t22770, t22771, t22773, t22774, t22776, t22777, t22779)
}
