//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2513;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta668(t11147: f64, t3584: f64, t45971: f64, t47774: f64, t11244: f64, t1661: f64, t43880: f64, t43889: f64, t14808: f64, t3279: f64, t11258: f64, t4748: f64, t14813: f64, t4764: f64, t11265: f64, t3271: f64, t4756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51002, t51004) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2513(t11147, t3584, t45971, t47774);
        let (t51007, t51010, t51012, t51014, t51016, t51018, t51021) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2514(t11244, t1661, t43880, t43889, t14808, t3279, t11258, t4748, t14813, t4764, t11265, t3271, t4756);
    (t51002, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021)
}
