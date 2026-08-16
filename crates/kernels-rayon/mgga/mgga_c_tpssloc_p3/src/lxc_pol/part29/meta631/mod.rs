//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2078;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta631(t16524: f64, t23896: f64, t45560: f64, t7769: f64, t16521: f64, t6534: f64, t1873: f64, t55405: f64, t23893: f64, t12524: f64, t26550: f64, t16535: f64, t7467: f64, t26135: f64, t3938: f64, t12816: f64, t191: f64, t192: f64, t2020: f64, t26161: f64, t26162: f64, t56404: f64, t16148: f64, t24995: f64, t8945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86639, t86642, t86646, t86651, t86653, t86655, t86660) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2078(t16524, t23896, t45560, t7769, t16521, t6534, t1873, t55405, t23893, t12524, t26550, t16535, t7467);
        let (t86668, t86673, t86676, t86679) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2079(t26135, t3938, t12816, t191, t192, t2020, t26161, t26162, t56404, t16148, t24995, t8945);
    (t86639, t86642, t86646, t86651, t86653, t86655, t86660, t86668, t86673, t86676, t86679)
}
