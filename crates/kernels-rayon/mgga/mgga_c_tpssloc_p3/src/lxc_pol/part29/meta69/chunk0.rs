//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 467/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk467(t1274: f64, t1276: f64, t1286: f64, t1288: f64, t1290: f64, t1293: f64, t1296: f64, t225: f64, t680: f64, t705: f64) -> f64 {
    let t1345 = (t680 + t705 + t1274 - t1276 + t1286 + t1288 + t1290 - t1293 - t1296) * t225;
    t1345
}
