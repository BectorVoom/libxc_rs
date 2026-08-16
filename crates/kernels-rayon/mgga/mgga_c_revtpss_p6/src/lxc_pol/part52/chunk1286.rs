//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1286/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1286(t2113: f64, t28271: f64, t28277: f64, t28974: f64, t572: f64, t7741: f64, t26733: f64, t129065: f64, t129069: f64, t129072: f64, t129078: f64, t129080: f64, t2040: f64, t28987: f64, t32373: f64, t5802: f64, t7557: f64, t7944: f64, t8725: f64) -> f64 {
    let t129082 = 6.0_f64 * t2113 * t28271;
    let t129084 = 6.0_f64 * t2113 * t28277;
    let t129089 = 6.0_f64 * t572 * t28974 * t7741;
    let t129092 = 6.0_f64 * t572 * t26733 * t7741;
    let t129093 = 6.0_f64 * t2040 * t28987 + 6.0_f64 * t5802 * t8725 + 3.0_f64 * t7557 * t7944 + t129065 + t129069 + t129072 + t129078 + t129080 + t129082 + t129084 + t129089 + t129092 + t32373;
    t129093
}
