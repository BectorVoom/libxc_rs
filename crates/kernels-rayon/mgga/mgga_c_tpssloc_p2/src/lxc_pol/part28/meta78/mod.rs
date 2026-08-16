//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk503;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk504;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk505;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk506;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta78(t1557: f64, t893: f64, t1541: f64, t917: f64, t1548: f64, t1551: f64, t1554: f64, t926: f64, t929: f64, t932: f64, t936: f64, t324: f64, t945: f64, t948: f64, t951: f64, t1545: f64, t300: f64, t311: f64, t924: f64, t943: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1559, t1561, t1568, t1569) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk503(t1557, t893, t1541, t917, t1548, t1551, t1554, t926, t929, t932);
        let t1573 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk504(t1541, t936);
        let (t1574, t1580) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk505(t1573, t324, t1541, t1548, t1551, t1554, t945, t948);
        let t1581 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk506(t1580, t951);
        let (t1585, t1587, t1589) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk507(t1545, t1559, t1561, t1569, t1574, t1581, t300, t311, t924, t943, t1580, t942, t951);
    (t1559, t1561, t1568, t1569, t1573, t1580, t1581, t1585, t1587, t1589)
}
