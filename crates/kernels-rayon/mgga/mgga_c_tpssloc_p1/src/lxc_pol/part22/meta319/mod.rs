//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1501;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta319(t11697: f64, t4953: f64, t3577: f64, t1229: f64, t3242: f64, t13969: f64, t4979: f64, t3506: f64, t4973: f64, t1227: f64, t11153: f64, t3584: f64, t1734: f64, t3508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15608, t15610, t15615, t15640, t15642, t15643, t15645, t15654) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1501(t11697, t4953, t3577, t1229, t3242, t13969, t4979, t3506, t4973, t1227, t11153, t3584);
        let t15659 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1502(t1734, t3508);
    (t15608, t15610, t15615, t15640, t15642, t15643, t15645, t15654, t15659)
}
