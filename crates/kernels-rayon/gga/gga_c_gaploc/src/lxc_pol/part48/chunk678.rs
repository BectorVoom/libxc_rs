//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 678/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk678(t3129: f64, t900: f64, t10615: f64, t9448: f64, t986: f64, t9438: f64, t2487: f64, t10318: f64, t544: f64, t9287: f64, t12964: f64, t2488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12968 = t900 * t3129;
    let t12969 = t10615 * t12968;
    let t12986 = t9448 * t986;
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12993 = t2488 * t12964;
    (t12968, t12969, t12986, t12987, t12988, t12990, t12991, t12993)
}
