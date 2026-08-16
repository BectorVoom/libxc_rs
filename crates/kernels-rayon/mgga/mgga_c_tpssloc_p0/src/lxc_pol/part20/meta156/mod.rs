//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk991;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk992;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk993;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk994;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk995;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk996;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk997;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta156(t570: f64, t515: f64, t25: f64, t1298: f64, t2249: f64, t3665: f64, t518: f64, zeta_threshold: f64, t28: f64, t1302: f64, t3231: f64, t3673: f64, t215: f64, t2559: f64, t535: f64, t1314: f64, t782: f64, t1317: f64, t2566: f64, t795: f64, t154: f64, t557: f64, t205: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3700, t3701) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk991(t570);
        let t3704 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk992(t515);
        let (t3710, t3711) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk993(t25, t1298, t2249, t3665, t3704, t518, zeta_threshold);
        let t3719 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk994(t28, t1302, t3231, t3673, t3711, t3710, zeta_threshold);
        let (t3725, t3726) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk995(t215, t2559, t535, t1314, t782);
        let (t3727, t3731, t3732) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk996(t1317, t3726, t2566, t535, t795, t154, t557);
        let t3733 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk997(t205, t3732);
        let t3734 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk998(t1307);
    (t3700, t3701, t3704, t3711, t3719, t3725, t3726, t3727, t3731, t3732, t3733, t3734)
}
