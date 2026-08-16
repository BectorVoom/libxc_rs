//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1297/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1297(t122820: f64, t127366: f64, t127369: f64, t127371: f64, t127373: f64, t127375: f64, t127378: f64, t128998: f64, t128999: f64, t129001: f64, t129008: f64, t2127: f64, t28586: f64, t28718: f64, t28939: f64, t7584: f64, t8065: f64, t8764: f64) -> f64 {
    let t131115 = -3.0_f64 * t122820 * t28718 - t2127 * t28586 + 3.0_f64 * t28939 * t8764 - t7584 * t8065 - t127366 - t127369 - t127371 - t127373 - t127375 - t127378 - 2.0_f64 * t128998 - 2.0_f64 * t128999 - 2.0_f64 * t129001 - 2.0_f64 * t129008;
    t131115
}
