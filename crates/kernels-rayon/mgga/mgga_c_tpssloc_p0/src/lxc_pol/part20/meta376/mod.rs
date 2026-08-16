//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1729;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta376(t2427: f64, t4202: f64, t9869: f64, t2655: f64, t4205: f64, t12914: f64, t12922: f64, t12926: f64, t12927: f64, t12928: f64, t12934: f64, t12942: f64, t12944: f64, t12947: f64, t9724: f64, t9780: f64, t9789: f64, t9863: f64, t1462: f64, t9912: f64, t9871: f64, t4101: f64, t9880: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13095, t13096, t13098, t13099) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1729(t2427, t4202, t9869, t2655, t4205, t12914, t12922, t12926, t12927, t12928, t12934, t12942, t12944, t12947, t9724, t9780, t9789, t9863);
        let (t13102, t13103, t13105, t13106, t13108, t13110, t13111) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1730(t1462, t9912, t9871, t2427, t4101, t9880, t2528, t4199, t2663, t4211, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890, t9894);
    (t13095, t13096, t13098, t13099, t13102, t13103, t13105, t13106, t13108, t13110, t13111)
}
