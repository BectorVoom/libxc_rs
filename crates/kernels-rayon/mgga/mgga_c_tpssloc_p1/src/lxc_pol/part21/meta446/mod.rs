//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1993;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta446(t1227: f64, t15643: f64, t11705: f64, t11719: f64, t11728: f64, t11734: f64, t11746: f64, t15610: f64, t15612: f64, t15617: f64, t15622: f64, t15627: f64, t15631: f64, t15637: f64, t15642: f64, t3490: f64, t3496: f64, t3506: f64, t3515: f64, t4974: f64, t4984: f64, t5019: f64, t12652: f64, t4972: f64, t4582: f64, t11153: f64, t3584: f64, t14165: f64, t1734: f64, t3508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15645, t15648) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1993(t1227, t15643, t11705, t11719, t11728, t11734, t11746, t15610, t15612, t15617, t15622, t15627, t15631, t15637, t15642, t3490, t3496, t3506, t3515, t4974, t4984, t5019);
        let (t15649, t15650, t15654, t15655, t15656, t15659) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1994(t12652, t4972, t4582, t11153, t3584, t14165, t1734, t3508);
    (t15645, t15648, t15649, t15650, t15654, t15655, t15656, t15659)
}
