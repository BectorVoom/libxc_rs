//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1679/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1679(t1426: f64, t545: f64, t2453: f64, t7283: f64, t25920: f64, t7063: f64, t3974: f64, t7259: f64, t2482: f64, t27: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25937 = t1426 * t545;
    let t25944 = t2453 * t7283;
    let t25949 = t25920 * t1426;
    let t25950 = t7063 * t25949;
    let t25969 = t7259 * t3974;
    let t25972 = t2482 * t7269 * t27;
    (t25937, t25944, t25949, t25950, t25969, t25972)
}
