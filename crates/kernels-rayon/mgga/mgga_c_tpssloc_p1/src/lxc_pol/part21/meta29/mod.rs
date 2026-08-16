//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk219;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk220;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk221;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk222;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk223;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta29(t544: f64, t564: f64, t254: f64, t563: f64, t144: f64, t193: f64, t523: f64, t525: f64, t533: f64, t113: f64, t510: f64, t513: f64, t111: f64, t112: f64, t11: f64, t2: f64, t10: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t566, t568) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk219(t544, t564, t254);
        let (t570, t571) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk220(t563, t568);
        let t574 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk221(t144, t193, t523, t525, t533, t571);
        let t576 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk222(t113, t510, t513, t574);
        let t577 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk223(t111, t112);
        let (t580, t581, t582, t583, t584) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk224(t576, t577, t11, t2, t10, t3);
    (t566, t568, t570, t571, t574, t576, t577, t580, t581, t582, t583, t584)
}
