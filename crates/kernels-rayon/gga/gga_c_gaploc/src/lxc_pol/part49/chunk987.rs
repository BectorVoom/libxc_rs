//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 987/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk987(t42960: f64, t29277: f64, t32607: f64, t9647: f64, t10639: f64, t16879: f64, t883: f64, t10736: f64, t7064: f64, t10635: f64, t2554: f64, t1841: f64, t3487: f64, t734: f64, t9641: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42961 = 0.4486140300916297499e-2_f64 * t42960;
    let t42963 = t9647 * t29277 * t32607;
    let t42964 = 0.76905262301422242837e-2_f64 * t42963;
    let t42967 = t9647 * t16879 * t883 * t10639;
    let t42968 = 0.38452631150711121417e-2_f64 * t42967;
    let t42970 = t7064 * t29277 * t10736;
    let t42971 = 0.12817543716903707139e-2_f64 * t42970;
    let t42973 = t7064 * t10635 * t2554;
    let t42974 = 0.64087718584518535698e-3_f64 * t42973;
    let t42978 = 0.85450291446024714263e-3_f64 * t1841 * t9641 * t3487 * t734;
    (t42961, t42964, t42968, t42971, t42974, t42978)
}
