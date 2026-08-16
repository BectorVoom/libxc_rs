//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk525;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk526;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta79<F: Float>(t1589: F, t959: F, t1409: F, t978: F, t977: F, t1554: F, t906: F, t340: F, t343: F, t974: F, t971: F, t973: F, t381: F, t998: F, t225: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1591, t1592, t1593, t1597) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk525::<F>(t1589, t959, t1409, t978, t977, t1554, t906);
        let (t1599, t1600, t1603) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk526::<F>(t1597, t340, t343, t974, t1593, t971, t973);
        let (t1604, t1606, t1607, t1610, t1611) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk527::<F>(t1603, t381, t1409, t998, t974, t225, t68);
    (t1591, t1592, t1593, t1597, t1599, t1600, t1603, t1604, t1606, t1607, t1610, t1611)
}
