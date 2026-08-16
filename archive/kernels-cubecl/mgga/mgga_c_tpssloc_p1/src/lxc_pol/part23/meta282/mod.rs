//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk976;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta282<F: Float>(t4295: F, t5617: F, t16891: F, t2645: F, t5591: F, t232: F, t5544: F, t4181: F, t1510: F, t4180: F, t20756: F, t820: F, t9607: F, t20857: F, t819: F, t20800: F, t847: F, t210: F, t214: F, t221: F, t4128: F, t12986: F, t13010: F, t13022: F, t16769: F, t16784: F, t16792: F, t16794: F, t4127: F, t787: F, t9540: F, t9559: F, t9572: F, t9579: F, t9583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20876, t20882, t20885, t20887, t20891, t20896) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk976::<F>(t4295, t5617, t16891, t2645, t5591, t232, t5544, t4181, t1510, t4180, t20756, t820, t9607);
        let (t20904, t20908, t20923, t20927, t20933, t20936) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk977::<F>(t20857, t819, t820, t20800, t847, t20756, t210, t214, t221, t4128, t5544, t12986, t13010, t13022, t16769, t16784, t16792, t16794, t4127, t787, t9540, t9559, t9572, t9579, t9583);
    (t20876, t20882, t20885, t20887, t20891, t20896, t20904, t20908, t20923, t20927, t20933, t20936)
}
