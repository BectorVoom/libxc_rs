//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 730/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk730(t1214: f64, t2142: f64, t7637: f64, t1269: f64, t2148: f64, t7635: f64) -> (f64, f64, f64) {
    let t7644 = t2142 * t1214;
    let t7645 = t7637 * t7644;
    let t7648 = t2148 * t1269;
    let t7651 = t2148 * t7635;
    (t7645, t7648, t7651)
}
