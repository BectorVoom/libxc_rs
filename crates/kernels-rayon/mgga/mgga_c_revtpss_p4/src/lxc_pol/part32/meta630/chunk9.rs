//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2040/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2040(t102888: f64, t107901: f64, t107919: f64, t107924: f64, t107930: f64, t107988: f64, t108009: f64, t108030: f64, t110177: f64, t110717: f64, t1113: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t27793: f64, t28291: f64, t28472: f64, t29953: f64, t29964: f64, t30420: f64, t4541: f64, t6416: f64, t7207: f64, t7428: f64, t7432: f64, t95976: f64) -> f64 {
    let t110954 = t1940 * t95976 * t29964 + 2.0_f64 * t28472 * t107924 - 3.0_f64 / 2.0_f64 * t26425 * t107919 - 3.0_f64 * t102888 * t27793 - 3.0_f64 * t26425 * t107930 + 3.0_f64 * t28291 * t108030 + t1940 * t7428 * t6416 / 2.0_f64 + t1940 * t30420 * t1113 / 2.0_f64 + 3.0_f64 * t2403 * t2071 * t107901 + 3.0_f64 / 2.0_f64 * t2403 * t7428 * t29953 - t1940 * t7432 * t107988 + 3.0_f64 * t4541 * t2071 * t108009 - t1940 * t110177 * t7207 / 2.0_f64 - t110717;
    t110954
}
