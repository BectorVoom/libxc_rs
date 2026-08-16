//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk615;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta101(t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64, t702: f64, t683: f64, t681: f64, t125: f64, t701: f64, t141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2405, t2406, t2408) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk615(t2388, t2391, t2394, t2398, t2400, t2403, t702, t683);
        let (t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk616(t681, t125, t701, t141);
    (t2405, t2406, t2408, t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417)
}
