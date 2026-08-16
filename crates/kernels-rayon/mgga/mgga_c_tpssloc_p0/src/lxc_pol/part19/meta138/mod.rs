//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk718;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk719;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta138(t1324: f64, t225: f64, t2600: f64, t541: f64, t1329: f64, t3726: f64, t119: f64, t3734: f64, t210: f64, t3719: f64, t3752: f64, t554: f64, t1332: f64, t68: f64, t1340: f64, t1333: f64, t1358: f64, t1362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3758, t3762, t3763, t3765, t3766, t3770, t3773) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk718(t1324, t225, t2600, t541, t1329, t3726, t119, t3734, t210, t3719, t3752);
        let (t3774, t3777) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk719(t3773, t554, t1332, t68);
        let (t3778, t3781, t3783) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk720(t1340, t3777, t1333, t1358, t1362);
    (t3758, t3762, t3763, t3765, t3766, t3770, t3773, t3774, t3777, t3778, t3781, t3783)
}
