//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1876;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1877;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta478(t20861: f64, t819: f64, t820: f64, t20853: f64, t232: f64, t5527: f64, t4181: f64, t9646: f64, t16839: f64, t2645: f64, t5591: f64, t1484: f64, t2632: f64, t5611: f64, t4180: f64, t119: f64, t20800: f64, t210: f64, t13251: f64, t16940: f64, t2630: f64, t2643: f64, t4167: f64, t4178: f64, t5593: f64, t5614: f64, t5619: f64, t787: f64, t817: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20963, t20969, t20972, t20974, t20978, t20981) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1876(t20861, t819, t820, t20853, t232, t5527, t4181, t9646, t16839, t2645, t5591, t1484, t2632);
        let (t20983, t20986) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1877(t16839, t20981, t2645, t2632, t5611);
        let (t20988, t20993, t20994, t20998) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1878(t20986, t4180, t4181, t119, t20800, t210, t13251, t16940, t20963, t20969, t20974, t20978, t20983, t2630, t2643, t4167, t4178, t5593, t5614, t5619, t787, t817);
    (t20963, t20969, t20972, t20974, t20978, t20983, t20986, t20988, t20993, t20994, t20998)
}
