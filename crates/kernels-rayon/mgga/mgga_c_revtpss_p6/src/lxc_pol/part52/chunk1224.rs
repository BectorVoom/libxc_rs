//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1224/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1224(t102851: f64, t102888: f64, t110165: f64, t127212: f64, t127566: f64, t127582: f64, t127907: f64, t1940: f64, t2403: f64, t26425: f64, t26585: f64, t27764: f64, t27799: f64, t27802: f64, t28460: f64, t32487: f64, t32491: f64, t32553: f64, t32559: f64, t32561: f64, t34151: f64, t34153: f64, t7207: f64, t7432: f64, t7862: f64) -> f64 {
    let t128150 = -3.0_f64 / 2.0_f64 * t102888 * t32553 - t1940 * t127582 * t7207 / 2.0_f64 + t110165 * t32559 + 3.0_f64 / 2.0_f64 * t2403 * t32487 * t7862 - t1940 * t7432 * t127212 / 2.0_f64 + t102851 * t34151 - t1940 * t28460 * t32561 / 2.0_f64 + 3.0_f64 * t26425 * t27799 * t127907 - t1940 * t26585 * t34153 / 2.0_f64 + 3.0_f64 * t127566 * t27764 - t1940 * t32491 * t27802 / 2.0_f64;
    t128150
}
