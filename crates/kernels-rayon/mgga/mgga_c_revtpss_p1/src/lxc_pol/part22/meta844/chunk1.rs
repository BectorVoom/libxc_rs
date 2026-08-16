//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2979/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2979(t40270: f64, t5737: f64, t13920: f64, t555: f64, t10073: f64, t14207: f64, t2782: f64, t4086: f64, t47973: f64, t543: f64, t10090: f64, t13805: f64, t1882: f64, t2482: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t49210 = t40270 * t5737;
    let t49213 = t555 * t13920;
    let t49238 = t10073 * t14207;
    let t49242 = t2782 * t4086 * t47973 * t543;
    let t49248 = t2482 * t10090 * t1882 * t13805 * t72 * t686;
    (t49210, t49213, t49238, t49242, t49248)
}
