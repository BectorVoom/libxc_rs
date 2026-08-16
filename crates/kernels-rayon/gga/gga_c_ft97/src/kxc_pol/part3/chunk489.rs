//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 489/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk489(t1095: f64, t236: f64, t3724: f64, t1096: f64, t709: f64, t680: f64, t688: f64, t2394: f64, t1092: f64, t458: f64, t2404: f64, t3691: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3725 = t236 * t1095;
    let t3726 = t3724 * t3725;
    let t3729 = t1096 * t709;
    let t3730 = t680 * t3729;
    let t3733 = t1096 * t688;
    let t3734 = t2394 * t3733;
    let t3738 = t458 * t1092;
    let t3740 = t2404 * t3691;
    (t3725, t3726, t3730, t3733, t3734, t3738, t3740)
}
