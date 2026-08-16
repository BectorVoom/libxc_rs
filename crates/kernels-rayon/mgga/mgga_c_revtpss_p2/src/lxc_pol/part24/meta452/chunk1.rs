//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1418/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1418(t10760: f64, t40763: f64, t4353: f64, t2710: f64, t4371: f64, t9732: f64, t4398: f64, t9323: f64, t4302: f64, t9586: f64, t9425: f64, t10565: f64, t1532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50611 = t10760 * t40763 * t4353;
    let t50703 = t2710 * t9732 * t4371;
    let t50852 = t4398 * t9323;
    let t50856 = t4302 * t9586;
    let t50888 = t4398 * t9425;
    let t50892 = t1532 * t10565;
    (t50611, t50703, t50852, t50856, t50888, t50892)
}
