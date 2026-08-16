//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 563/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk563(t2089: f64, t3234: f64, t723: f64, t1445: f64, t2004: f64, t2087: f64, t2103: f64, t2638: f64, t5974: f64, t6141: f64, t780: f64, t807: f64, t9942: f64, t9946: f64, t9947: f64, t9950: f64, t9955: f64, t9958: f64, t9961: f64, t9966: f64, t9969: f64, t9972: f64) -> f64 {
    let t9975 = t2089 * t3234;
    let t9976 = t9975 * t723;
    let t9977 = t1445 * t9976;
    let t9980 = -t9942 - t9946 + 0.23005755572352449806e1_f64 * t807 * t9947 + 0.35750489951850426669e0_f64 * t2004 * t9950 + 0.46011511144704899612e1_f64 * t807 * t9955 - 0.71500979903700853338e0_f64 * t6141 * t9958 + 0.14300195980740170668e1_f64 * t2103 * t9961 + 0.35750489951850426669e0_f64 * t780 * t9966 + 0.10725146985555128001e1_f64 * t9969 * t5974 - 0.21450293971110256002e1_f64 * t2638 * t9972 - 0.69017266717057349418e1_f64 * t2087 * t9977;
    t9980
}
