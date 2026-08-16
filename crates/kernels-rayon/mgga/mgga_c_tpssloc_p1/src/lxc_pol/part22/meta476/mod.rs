//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1872;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta476(t16891: f64, t2645: f64, t5591: f64, t232: f64, t5544: f64, t4181: f64, t1510: f64, t4180: f64, t20756: f64, t820: f64, t9607: f64, t20857: f64, t819: f64, t20800: f64, t847: f64, t210: f64, t214: f64, t221: f64, t4128: f64, t12986: f64, t13010: f64, t13022: f64, t16769: f64, t16784: f64, t16792: f64, t16794: f64, t4127: f64, t787: f64, t9540: f64, t9559: f64, t9572: f64, t9579: f64, t9583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20882, t20885, t20887, t20891, t20896, t20904) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1872(t16891, t2645, t5591, t232, t5544, t4181, t1510, t4180, t20756, t820, t9607, t20857, t819);
        let (t20908, t20923, t20927, t20933, t20936) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1873(t20800, t820, t847, t20756, t210, t214, t221, t4128, t5544, t12986, t13010, t13022, t16769, t16784, t16792, t16794, t4127, t787, t9540, t9559, t9572, t9579, t9583);
    (t20882, t20885, t20887, t20891, t20896, t20904, t20908, t20923, t20927, t20933, t20936)
}
