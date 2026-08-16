//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 785/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk785(t12360: f64, t2312: f64, t2321: f64, t882: f64, t9493: f64, t2325: f64, t29661: f64, t883: f64, t2326: f64, t9074: f64, t9079: f64, t2317: f64, t6525: f64, t9066: f64) -> (f64, f64, f64, f64, f64) {
    let t39791 = t2312 * t12360;
    let t39794 = t882 * t9493 * t2321;
    let t39798 = t882 * t2325 * t883 * t29661;
    let t39805 = t9074 * t9079 * t2326;
    let t39808 = t6525 * t9066 * t2317;
    (t39791, t39794, t39798, t39805, t39808)
}
