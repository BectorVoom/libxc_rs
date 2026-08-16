//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1229/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1229(t114107: f64, t114110: f64, t114113: f64, t114117: f64, t114128: f64, t114140: f64, t114150: f64, t114188: f64, t115748: f64, t1940: f64, t2071: f64, t22783: f64, t2403: f64, t26425: f64, t28291: f64, t28460: f64, t29939: f64, t29949: f64, t29967: f64, t29970: f64, t30420: f64, t33: f64, t4541: f64, t7432: f64, t7862: f64, t8020: f64, t95964: f64) -> f64 {
    let t115913 = 9.0_f64 * t28291 * t114128 - 3.0_f64 * t1940 * t28460 * t29967 - 3.0_f64 / 2.0_f64 * t1940 * t28460 * t29970 - t1940 * t7432 * t114150 / 2.0_f64 - 3.0_f64 * t1940 * t95964 * t114188 + 9.0_f64 * t4541 * t8020 * t29939 + 9.0_f64 / 2.0_f64 * t2403 * t30420 * t7862 - 9.0_f64 / 2.0_f64 * t26425 * t114110 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t114113 - 9.0_f64 / 2.0_f64 * t26425 * t114107 + t1940 * t2071 * t22783 / 2.0_f64 + 9.0_f64 * t4541 * t2071 * t114117 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t114140 + t1940 * t115748 * t33 / 2.0_f64 + 9.0_f64 * t2403 * t8020 * t29949;
    t115913
}
