//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1645;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta451(t22692: f64, t3851: f64, t7208: f64, t22717: f64, t22725: f64, t1332: f64, t1336: f64, t2089: f64, t22697: f64, t22701: f64, t22707: f64, t22721: f64, t22728: f64, t22730: f64, t3773: f64, t3777: f64, t7209: f64, t7211: f64, t1338: f64, t7191: f64, t1352: f64, t24063: f64, t553: f64, t2085: f64, t3787: f64, t3793: f64, t3856: f64, t22735: f64, t22743: f64, t22745: f64, t22749: f64, t22752: f64, t22884: f64, t22888: f64, t22895: f64, t22900: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24099, t24103, t24108, t24110, t24115) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1645(t22692, t3851, t7208, t22717, t22725, t1332, t1336, t2089, t22697, t22701, t22707, t22721, t22728, t22730, t3773, t3777, t7209, t7211);
        let (t24116, t24117, t24121, t24128, t24131, t24137) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1646(t1338, t7191, t1352, t24063, t553, t2085, t3787, t3793, t3856, t7208, t1336, t22735, t22743, t22745, t22749, t22752, t22884, t22888, t22895, t22900, t544);
    (t24099, t24103, t24108, t24110, t24115, t24116, t24117, t24121, t24128, t24131, t24137)
}
