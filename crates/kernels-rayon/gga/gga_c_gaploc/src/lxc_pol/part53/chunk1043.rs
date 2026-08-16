//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1043/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1043(t43028: f64, t43032: f64, t43040: f64, t43043: f64, t43049: f64, t43053: f64, t43054: f64, t43055: f64, t43094: f64, t47629: f64, t47631: f64, t47634: f64, t47636: f64, t47640: f64, t47644: f64, t47646: f64, t47650: f64, t47652: f64) -> f64 {
    let t51013 = t43028 + t43032 - t43040 - t47629 + t47631 - t47634 + t47636 + t43043 - t43049 - t43053 + t43054 - t43055 + 0.20508069947045931423e-1_f64 * t47640 + t47644 + 0.46143157380853345702e-1_f64 * t47646 - 0.30762104920568897134e-1_f64 * t47650 + 0.85450291446024714264e-3_f64 * t47652 + t43094;
    t51013
}
