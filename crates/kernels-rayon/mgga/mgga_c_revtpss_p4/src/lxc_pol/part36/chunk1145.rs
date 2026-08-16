//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1145/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1145(t7760: f64, t786: f64, t789: f64, t27198: f64, t867: f64, t7060: f64, t7063: f64, t14685: f64, t1941: f64, t25245: f64, t4430: f64, t1561: f64, t25266: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27202 = t786 * t7760;
    let t27203 = t27202 * t789;
    let t27212 = t27198 * t867;
    let t27213 = t786 * t27212;
    let t27214 = t27213 * t7060;
    let t27216 = t7063 * t27212;
    let t27217 = t27216 * t7060;
    let t27221 = t1941 * t14685;
    let t27228 = t25245 * t4430;
    let t27230 = t25266 * t1561;
    (t27202, t27203, t27212, t27213, t27214, t27216, t27217, t27221, t27228, t27230)
}
