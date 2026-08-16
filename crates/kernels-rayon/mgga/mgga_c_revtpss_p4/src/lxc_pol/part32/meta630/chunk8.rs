//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2039/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2039(t107882: f64, t107885: f64, t107895: f64, t107939: f64, t107943: f64, t107947: f64, t107985: f64, t108028: f64, t108036: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t27773: f64, t27777: f64, t27810: f64, t27817: f64, t28460: f64, t28472: f64, t29949: f64, t30420: f64, t7200: f64, t7428: f64, t8020: f64) -> f64 {
    let t110920 = 3.0_f64 * t2403 * t8020 * t27777 + 2.0_f64 * t28472 * t108036 + t28472 * t108028 + 3.0_f64 * t2403 * t8020 * t27773 - 3.0_f64 * t28472 * t107947 - 3.0_f64 * t26425 * t107985 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t107939 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t107943 + 3.0_f64 * t2403 * t7428 * t29949 - 3.0_f64 / 2.0_f64 * t26425 * t107882 + 3.0_f64 * t2403 * t8020 * t27810 + 3.0_f64 / 2.0_f64 * t2403 * t30420 * t7200 - 3.0_f64 * t26425 * t107895 - 3.0_f64 * t26425 * t107885 - t1940 * t28460 * t27817;
    t110920
}
