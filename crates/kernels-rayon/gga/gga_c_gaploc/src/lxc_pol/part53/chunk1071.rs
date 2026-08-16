//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1071/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1071(t12849: f64, t12850: f64, t12851: f64, t12853: f64, t12858: f64, t12864: f64, t14454: f64, t14455: f64, t14456: f64, t14457: f64, t14480: f64, t14522: f64, t51232: f64, t51234: f64, t7: f64) -> f64 {
    let tv4rhosigma318 = t14454 - t12851 - t14455 + t12853 - t14456 + t12849 - t12858 + t14457 - t12850 + t12864 - t14480 + t14522 + t7 * (t51232 + t51234);
    tv4rhosigma318
}
