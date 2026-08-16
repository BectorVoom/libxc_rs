//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1811;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1812;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1813;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta369(t13563: f64, t13566: f64, t4348: f64, t690: f64, t12606: f64, t883: f64, t882: f64, t123: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10577: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1811(t13563, t13566, t4348, t690);
        let (t13603, t13611) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1812(t13602, t12606, t883);
        let (t13612, t13613) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1813(t13611, t882, t123);
        let t13615 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1814(t10556, t10558, t10560, t10562, t10577, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13600, t13601, t13603, t13613);
    (t13600, t13601, t13602, t13603, t13611, t13612, t13613, t13615)
}
