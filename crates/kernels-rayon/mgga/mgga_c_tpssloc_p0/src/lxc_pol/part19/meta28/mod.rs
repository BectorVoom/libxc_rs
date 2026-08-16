//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk215;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk216;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk217;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk218;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk219;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk220;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta28(t219: f64, t541: f64, t555: f64, t559: f64, t539: f64, t553: f64, t544: f64, t254: f64, t144: f64, t193: f64, t523: f64, t525: f64, t533: f64, t113: f64, t510: f64, t513: f64, t111: f64, t112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t562 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk215(t219, t541, t555, t559);
        let (t563, t564) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk216(t539, t562, t553);
        let (t566, t568) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk217(t544, t564, t254);
        let (t570, t571) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk218(t563, t568);
        let t574 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk219(t144, t193, t523, t525, t533, t571);
        let t576 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk220(t113, t510, t513, t574);
        let t577 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk221(t111, t112);
    (t562, t563, t564, t566, t568, t570, t571, t574, t576, t577)
}
