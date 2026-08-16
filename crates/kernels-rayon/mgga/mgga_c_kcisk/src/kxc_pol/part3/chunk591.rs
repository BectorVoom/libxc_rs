//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 591/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk591(t4597: f64, t662: f64, t3290: f64, t5006: f64, t649: f64, t654: f64, t3934: f64, t1774: f64, t9: f64, t1782: f64, t1636: f64, t1785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5007 = t662 * t4597;
    let t5008 = t5007 * t3290;
    let t5009 = t5006 * t5008;
    let t5012 = t649 * t654;
    let t5013 = t5012 * t3934;
    let t5014 = t9 * t1774;
    let t5015 = t5014 * t1782;
    let t5016 = t1636 * t1785;
    (t5007, t5008, t5009, t5013, t5014, t5015, t5016)
}
