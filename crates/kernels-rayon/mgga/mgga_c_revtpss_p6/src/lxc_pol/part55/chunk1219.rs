//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1219/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1219(t127892: f64, t892: f64, t34097: f64, t775: f64, t121716: f64, t125985: f64, t126018: f64, t127593: f64, t127596: f64, t1940: f64, t25207: f64, t26425: f64, t26585: f64, t27383: f64, t27387: f64, t28472: f64, t30: f64, t32491: f64, t33740: f64, t34090: f64, t7787: f64, t92790: f64, t98763: f64) -> (f64, f64, f64) {
    let t127893 = t127892 * t892;
    let t127907 = t34097 * t775;
    let t127912 = t28472 * t27383 * t127593 - 3.0_f64 / 2.0_f64 * t26425 * t25207 * t127596 - t1940 * t32491 * t27387 / 2.0_f64 - t1940 * t26585 * t33740 / 2.0_f64 + t1940 * t127893 * t30 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t92790 * t34090 - t1940 * t121716 * t7787 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t125985 + t28472 * t98763 * t34097 + 3.0_f64 * t26425 * t27383 * t127907 + t28472 * t126018;
    (t127893, t127907, t127912)
}
