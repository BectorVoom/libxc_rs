//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2441;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta647(t340: f64, t625: f64, t221: f64, t339: f64, t344: f64, t1887: f64, t2262: f64, t337: f64, t13783: f64, t984: f64, t10277: f64, t343: f64, t3014: f64, t4509: f64, t42308: f64, t974: f64, t10224: f64, t2999: f64, t973: f64, t2978: f64, t698: f64, t2981: f64, t2402: f64, t976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42813, t42817, t42830, t42837, t42841) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2441(t340, t625, t221, t339, t344, t1887, t2262, t337, t13783, t984, t10277, t343);
        let (t42846, t42861, t42873, t42875, t42877, t42891) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2442(t3014, t4509, t42308, t974, t10224, t2999, t973, t2978, t698, t2981, t2402, t976);
    (t42813, t42817, t42830, t42837, t42841, t42846, t42861, t42873, t42875, t42877, t42891)
}
