//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1817;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta479(t1089: f64, t1240: f64, t1251: f64, t607: f64, t24601: f64, t225: f64, t3590: f64, t497: f64, t462: f64, t3597: f64, t3599: f64, t7300: f64, t2123: f64, t3471: f64, t11613: f64, t1238: f64, t2121: f64, t2155: f64, t24564: f64, t24568: f64, t24571: f64, t24575: f64, t24577: f64, t24582: f64, t24587: f64, t24589: f64, t24591: f64, t24597: f64, t3487: f64, t3593: f64, t3600: f64, t7283: f64, t7351: f64, t7356: f64, t7392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24602, t24603, t24604, t24605, t24611, t24612, t24615) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1817(t1089, t1240, t1251, t607, t24601, t225, t3590, t497, t462, t3597);
        let (t24616, t24617, t24626, t24629) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1818(t24615, t3599, t7300, t2123, t3471, t11613, t1238, t2121, t2155, t24564, t24568, t24571, t24575, t24577, t24582, t24587, t24589, t24591, t24597, t24605, t24612, t3487, t3593, t3600, t7283, t7351, t7356, t7392);
    (t24602, t24603, t24604, t24605, t24611, t24612, t24615, t24616, t24617, t24626, t24629)
}
