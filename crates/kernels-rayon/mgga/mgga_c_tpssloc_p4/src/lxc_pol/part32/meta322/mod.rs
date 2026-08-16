//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1351;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1352;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta322(t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64, t1190: f64, t3030: f64, t3032: f64, t3505: f64, t10469: f64, t466: f64, t10471: f64, t1208: f64, t478: f64, t10477: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11668, t11678, t11692, t11697, t11707, t11708, t11709, t11712) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1351(t3585, t820, t10401, t3575, t3610, t3624, t3521, t1190, t3030, t3032, t3505, t10469, t466);
        let t11713 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1352(t10471, t11712);
        let (t11715, t11716, t11717) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1353(t1208, t478, t10477, t483);
    (t11668, t11678, t11692, t11697, t11707, t11708, t11709, t11712, t11713, t11715, t11716, t11717)
}
