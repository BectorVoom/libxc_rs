//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 679/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk679(t9448: f64, t986: f64, t9438: f64, t2487: f64, t10318: f64, t544: f64, t9287: f64, t10268: f64, t2365: f64, t4391: f64, t2097: f64, t3039: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12986 = t9448 * t986;
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t13045 = t3039 * t2097;
    (t12986, t12987, t12988, t12990, t12991, t12996, t12997, t13045)
}
