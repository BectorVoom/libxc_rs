//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1890;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1891;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta405(t4680: f64, t4684: f64, t11060: f64, t3040: f64, t1629: f64, t4673: f64, t1049: f64, t4649: f64, t1060: f64, t11066: f64, t1615: f64, t3166: f64, t4677: f64, t1625: f64, t3120: f64, t14506: f64, t3199: f64, t1058: f64, t11034: f64, t11051: f64, t11059: f64, t11065: f64, t14572: f64, t1630: f64, t1632: f64, t3076: f64, t3180: f64, t3186: f64, t3193: f64, t3200: f64, t3202: f64, t4669: f64, t4674: f64, t4678: f64, t4681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14574, t14577, t14578, t14581, t14587, t14590, t14591, t14595) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1890(t4680, t4684, t11060, t3040, t1629, t4673, t1049, t4649, t1060, t11066, t1615, t3166);
        let (t14596, t14600, t14606, t14608) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1891(t1060, t14595, t4673, t4677, t1625, t3120, t14506, t3199);
        let t14613 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1892(t1058, t11034, t11051, t11059, t11065, t14572, t14574, t14578, t14581, t14587, t14591, t14596, t14600, t14606, t14608, t1630, t1632, t3076, t3180, t3186, t3193, t3200, t3202, t4669, t4674, t4678, t4681);
    (t14574, t14577, t14578, t14581, t14587, t14590, t14591, t14596, t14600, t14606, t14608, t14613)
}
