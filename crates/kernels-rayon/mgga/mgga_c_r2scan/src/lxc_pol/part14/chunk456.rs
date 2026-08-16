//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 456/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk456(t1356: f64, t1358: f64, t1360: f64, t1378: f64, t1387: f64, t1389: f64, t1413: f64, t1418: f64, t1783: f64, t2045: f64, t2052: f64, t2059: f64, t2063: f64, t2065: f64, t2068: f64, t246: f64, t765: f64) -> f64 {
    let t2073 = 0.571528e-1_f64 * t2045 + t2052 - t2059 - 0.1350520664e0_f64 * t2063 + t1356 - t1358 - t1360 - t1378 + 0.675260332e-1_f64 * t765 * t2065 + 0.1350520664e0_f64 * t765 * t2068 + t1387 + t1389 + t1413 - 0.285764e-1_f64 * t246 * t1783 - t1418;
    t2073
}
