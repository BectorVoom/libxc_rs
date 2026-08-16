//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 643/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk643(t11341: f64, t11381: f64, t11421: f64, t11451: f64, t11465: f64, t11499: f64, t11535: f64, t11553: f64, t9664: f64, t9666: f64, t9669: f64, t9672: f64, t9674: f64, t9676: f64) -> (f64, f64) {
    let t11556 = t11341 + t11381 + t11421 + t11451 + t11465 + t11499 + t11535 + t11553;
    let t11568 = -21.0_f64 / 128.0_f64 * t9664 + 147.0_f64 / 4096.0_f64 * t9666 - 63.0_f64 / 262144.0_f64 * t9669 + 21.0_f64 / 262144.0_f64 * t9672 - 49.0_f64 / 4096.0_f64 * t9674 + 7.0_f64 / 128.0_f64 * t9676;
    (t11556, t11568)
}
