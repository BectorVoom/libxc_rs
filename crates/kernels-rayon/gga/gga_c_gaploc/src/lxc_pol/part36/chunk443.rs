//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 443/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk443(t1645: f64, t1987: f64, t121: f64, t2084: f64, t1: f64, t313: f64, t191: f64, t835: f64, t830: f64, t106: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5974 = t1645 * t1987;
    let t6058 = t121 * t2084;
    let t6059 = t6058 * t1;
    let t6060 = t313 * t6059;
    let t6066 = t191 * t835;
    let t6109 = t830 * t1;
    let t6110 = t6109 * t106;
    let t6111 = t787 * t6110;
    (t5974, t6058, t6059, t6060, t6066, t6109, t6110, t6111)
}
