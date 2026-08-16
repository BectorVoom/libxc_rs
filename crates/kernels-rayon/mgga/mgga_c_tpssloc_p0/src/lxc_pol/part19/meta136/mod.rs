//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk710;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk711;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk712;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta136(t17: f64, t3696: f64, t1388: f64, t25: f64, t28: f64, t570: f64, t515: f64, t1298: f64, t2249: f64, t3665: f64, t518: f64, t1302: f64, t3231: f64, t3673: f64, zeta_threshold: f64, t215: f64, t2559: f64, t535: f64, t1314: f64, t782: f64, t1317: f64, t2566: f64, t795: f64, t154: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3697, t3698) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk710(t17, t3696, t1388);
        let (t3700, t3701, t3704, t3711, t3719) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk711(t25, t28, t570, t515, t1298, t2249, t3665, t518, t1302, t3231, t3673, zeta_threshold);
        let (t3725, t3726) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk712(t215, t2559, t535, t1314, t782);
        let (t3727, t3731, t3732) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk713(t1317, t3726, t2566, t535, t795, t154, t557);
    (t3697, t3698, t3700, t3701, t3704, t3711, t3719, t3725, t3726, t3727, t3731, t3732)
}
