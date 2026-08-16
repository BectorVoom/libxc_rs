//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 956/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk956(t1940: f64, t2071: f64, t2255: f64, t1468: f64, t2403: f64, t26425: f64, t26585: f64, t27160: f64, t27166: f64, t27169: f64, t27173: f64, t27376: f64, t27385: f64, t27387: f64, t27391: f64, t27395: f64, t27402: f64, t28291: f64, t28456: f64, t28460: f64, t28472: f64, t30: f64, t605: f64, t7010: f64, t7092: f64, t7428: f64, t7432: f64, t7749: f64, t7787: f64, t8020: f64) -> (f64, f64) {
    let t28490 = t1940 * t2071 * t2255;
    let t28491 = 3.0_f64 * t28291 * t27160 + 3.0_f64 / 2.0_f64 * t2403 * t7428 * t7749 - 3.0_f64 / 2.0_f64 * t26425 * t27166 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t27169 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t27173 + 3.0_f64 / 2.0_f64 * t2403 * t8020 * t7010 + t1940 * t28456 * t30 / 2.0_f64 - t1940 * t28460 * t7092 / 2.0_f64 + t1940 * t8020 * t605 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t27376 - t1940 * t26585 * t7787 / 2.0_f64 + t28472 * t27385 - t1940 * t7432 * t27387 / 2.0_f64 - t1940 * t7432 * t27391 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t27395 + t1940 * t7428 * t1468 / 2.0_f64 - t1940 * t7432 * t27402 / 2.0_f64 + t28490;
    (t28490, t28491)
}
