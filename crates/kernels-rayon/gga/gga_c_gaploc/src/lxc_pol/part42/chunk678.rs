//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 678/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk678(t12939: f64, t587: f64, t10608: f64, t3177: f64, t9272: f64, t993: f64, t9263: f64, t2890: f64, t9267: f64, t3129: f64, t900: f64, t10615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12940 = t587 * t12939;
    let t12953 = t10608 * t3177;
    let t12954 = t9272 * t12953;
    let t12957 = t993 * t3177;
    let t12958 = t9263 * t12957;
    let t12960 = t2890 * t3177;
    let t12961 = t9267 * t12960;
    let t12968 = t900 * t3129;
    let t12969 = t10615 * t12968;
    (t12940, t12953, t12954, t12957, t12958, t12960, t12961, t12968, t12969)
}
