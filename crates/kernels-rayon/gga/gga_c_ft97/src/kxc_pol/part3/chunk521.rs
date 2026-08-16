//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 521/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk521(t1196: f64, t816: f64, t820: f64, t1095: f64, t2697: f64, t274: f64, t688: f64, t3750: f64, t801: f64, t231: f64, t1193: f64, t278: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4064 = t816 * t1196;
    let t4065 = t4064 * t820;
    let t4068 = t2697 * t1095;
    let t4069 = t274 * t688;
    let t4072 = t801 * t3750;
    let t4073 = t4072 * t274;
    let t4075 = t1095 * t688;
    let t4077 = t231 * t4075 * t274;
    let t4080 = t1193 * t688;
    let t4083 = t3750 * t278;
    (t4064, t4065, t4068, t4069, t4072, t4073, t4077, t4080, t4083)
}
