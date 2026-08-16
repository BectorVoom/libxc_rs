//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1121/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1121(t1426: f64, t3999: f64, t1962: f64, t198: f64, t205: f64, t30: f64, t892: f64, t689: f64, t7774: f64, t25411: f64, t213: f64, t7759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26079 = t1426 * t3999;
    let t27158 = t198 * t205 * t1962;
    let t27159 = t892 * t30;
    let t27186 = t7774 * t689;
    let t27187 = t25411 * t27186;
    let t27189 = t213 * t7759;
    (t26079, t27158, t27159, t27186, t27187, t27189)
}
