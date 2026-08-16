//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 833/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk833(t4282: f64, t829: f64, t1519: f64, t814: f64, t235: f64, t4265: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t2617: f64, t4162: f64, t4166: f64, t4281: f64, t4283: f64, t4286: f64, t4288: f64, t4291: f64, t808: f64, t812: f64, t861: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t4292 = t4282 * t829;
    let t4295 = t814 * t1519;
    let t4296 = t4295 * t829;
    let t4298 = t235 * t4265;
    let t4300 = t1499 * t863 - t1523 * t2617 + t1525 * t808 + t226 * t4298 + t255 * t4162 - t4166 * t861 + 2.0_f64 * t4281 * t4283 - t4286 * t812 - t4288 * t812 - t4291 * t4292 - t4296 * t812;
    (t4292, t4295, t4296, t4298, t4300)
}
