//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk705;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk706;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk707;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk708;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta135(t25: f64, t2249: f64, t3664: f64, t3665: f64, t514: f64, t528: f64, t1081: f64, zeta_threshold: f64, t28: f64, t3231: f64, t517: f64, t157: f64, t182: f64, t118: f64, t521: f64, t2375: f64, t1294: f64, t2371: f64, t2528: f64, t1284: f64, t172: f64, t763: f64, t2535: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3671, t3672, t3673) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk705(t25, t2249, t3664, t3665, t514, t528, t1081, zeta_threshold);
        let t3681 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk706(t28, t3231, t3672, t3673, t517, t157, t3671, zeta_threshold);
        let (t3683, t3684) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk707(t182, t3681, t118, t521);
        let (t3686, t3688, t3690, t3691) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk708(t2375, t3684, t1294, t2371, t2528, t1284, t172);
        let (t3693, t3695, t3696) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk709(t3691, t763, t1294, t2535, t184, t3681);
    (t3672, t3673, t3681, t3683, t3684, t3686, t3688, t3690, t3691, t3693, t3695, t3696)
}
