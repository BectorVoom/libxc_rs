//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 825/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk825(t27202: f64, t789: f64, t231: f64, t7759: f64, t836: f64, t7076: f64, t27198: f64, t867: f64, t786: f64, t7060: f64, t7063: f64, t14685: f64, t1941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27203 = t27202 * t789;
    let t27206 = t7759 * t836 * t231;
    let t27207 = t7076 * t27206;
    let t27212 = t27198 * t867;
    let t27213 = t786 * t27212;
    let t27214 = t27213 * t7060;
    let t27216 = t7063 * t27212;
    let t27217 = t27216 * t7060;
    let t27221 = t1941 * t14685;
    (t27203, t27206, t27207, t27214, t27217, t27221)
}
