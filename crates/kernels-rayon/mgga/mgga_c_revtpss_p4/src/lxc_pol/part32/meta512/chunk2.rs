//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1807/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1807(t1468: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t26590: f64, t28460: f64, t29599: f64, t29602: f64, t29606: f64, t29713: f64, t29716: f64, t29719: f64, t30: f64, t30317: f64, t30420: f64, t4541: f64, t5824: f64, t7432: f64, t7749: f64, t7787: f64, t8020: f64) -> f64 {
    let t30438 = 3.0_f64 * t4541 * t30317 + 3.0_f64 * t2403 * t8020 * t7749 - 3.0_f64 * t26425 * t29599 + 3.0_f64 * t2403 * t2071 * t29602 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t29606 + t1940 * t30420 * t30 / 2.0_f64 - t1940 * t28460 * t7787 + t1940 * t8020 * t1468 + t1940 * t26590 * t29713 - t1940 * t7432 * t29716 - t1940 * t7432 * t29719 / 2.0_f64 + t1940 * t2071 * t5824 / 2.0_f64;
    t30438
}
