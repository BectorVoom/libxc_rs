//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 887/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk887(t21502: f64, t42944: f64, t1841: f64, t21501: f64, t13182: f64, t2563: f64, t3487: f64, t7284: f64, t9647: f64, t29277: f64, t32607: f64, t10639: f64, t16879: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42945 = t21502 * t42944;
    let t42948 = 0.51270174867614828557e-2_f64 * t1841 * t21501 * t42945;
    let t42953 = t1841 * t13182;
    let t42954 = 0.85450291446024714264e-3_f64 * t42953;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    let t42961 = 0.4486140300916297499e-2_f64 * t42960;
    let t42963 = t9647 * t29277 * t32607;
    let t42967 = t9647 * t16879 * t883 * t10639;
    (t42945, t42948, t42954, t42961, t42963, t42967)
}
