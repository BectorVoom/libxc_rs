//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 812/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk812(t2009: f64, t9208: f64, t9189: f64, t9184: f64, t4998: f64, t9213: f64, t2013: f64, t2630: f64, t7624: f64, t4419: f64, t9227: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24976 = t9208 * t2009;
    let t24978 = t9189 * t2009;
    let t24980 = t9184 * t2009;
    let t25006 = t4998 * t9213;
    let t25007 = t2013 * t25006;
    let t25024 = t2630 * t7624;
    let t25026 = t4419 * t9227;
    let t25027 = t782 * t25026;
    (t24976, t24978, t24980, t25007, t25024, t25027)
}
