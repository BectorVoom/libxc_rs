//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk508;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk509;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta79(t1589: f64, t959: f64, t1409: f64, t978: f64, t977: f64, t1554: f64, t906: f64, t340: f64, t343: f64, t974: f64, t971: f64, t973: f64, t381: f64, t998: f64, t225: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1591, t1592, t1593, t1597) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk508(t1589, t959, t1409, t978, t977, t1554, t906);
        let (t1599, t1600, t1603) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk509(t1597, t340, t343, t974, t1593, t971, t973);
        let (t1604, t1606, t1607, t1610, t1611) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk510(t1603, t381, t1409, t998, t974, t225, t68);
    (t1591, t1592, t1593, t1597, t1599, t1600, t1603, t1604, t1606, t1607, t1610, t1611)
}
