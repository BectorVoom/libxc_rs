//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1703;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1704;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta446(t22751: f64, t6970: f64, t3777: f64, t6944: f64, t3787: f64, t59: f64, t240: f64, t1336: f64, t6943: f64, t835: f64, t1354: f64, t6604: f64, t6919: f64, t6937: f64, t6950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22752, t22756, t22759, t22760, t22761, t22764, t22765) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1703(t22751, t6970, t3777, t6944, t3787, t59, t240, t1336, t6943, t835);
        let (t22766, t22779) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1704(t1354, t22765, t6604, t6919);
        let (t22780, t22782, t22783) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1705(t22779, t6937, t6950, t835, t1336);
    (t22752, t22756, t22759, t22760, t22761, t22764, t22765, t22766, t22779, t22780, t22782, t22783)
}
