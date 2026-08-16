//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 397/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk397(t1315: f64, t1327: f64, t1341: f64, t1360: f64, t1363: f64, t1811: f64, t1815: f64, t1827: f64, t1831: f64, t559: f64) -> f64 {
    let t1834 = -t1327 - t1315 * t1811 / 48.0_f64 + t1815 * t559 / 3072.0_f64 - t1341 * t1827 / 3072.0_f64 - t1360 - t1363 * t1831 / 768.0_f64;
    t1834
}
