//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk675;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk676;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk677;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk678;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk679;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta96(t2405: f64, t702: f64, t683: f64, t681: f64, t125: f64, t701: f64, t141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2406, t2408) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk675(t2405, t702, t683);
        let t2409 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk676(t681);
        let (t2410, t2411) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk677(t2409, t125);
        let t2412 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk678(t701);
        let (t2413, t2414) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk679(t141);
        let (t2415, t2417) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk680(t2412, t2414, t2411);
    (t2406, t2408, t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417)
}
