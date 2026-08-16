//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1742;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1743;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1744;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta382(t776: f64, t828: f64, t13228: f64, t13222: f64, t1500: f64, t2693: f64, t4163: f64, t838: f64, t120: f64, t4233: f64, t4180: f64, t4182: f64, t4181: f64, t9632: f64, t2642: f64, t4166: f64, t2617: f64, t4177: f64, t2628: f64, t836: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13229, t13231, t13234, t13237, t13242) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1742(t776, t828, t13228, t13222, t1500, t2693, t4163, t838, t120, t4233);
        let (t13244, t13248, t13251) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1743(t13242, t4180, t4182, t4181, t9632, t2642, t4166);
        let t13254 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1744(t2617, t4177);
        let (t13257, t13258) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1745(t2628, t836, t812);
    (t13229, t13231, t13234, t13237, t13242, t13244, t13248, t13251, t13254, t13257, t13258)
}
