//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 855/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk855(t12823: f64, t15737: f64, t15746: f64, t3499: f64, t16732: f64, t2102: f64, t16682: f64, t1775: f64, t4762: f64, t16687: f64, t9217: f64, t16694: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17296 = t12823 * t15737;
    let t17299 = t3499 * t15746;
    let t17302 = t2102 * t16732;
    let t17305 = t2102 * t16682;
    let t17310 = t1775 * t4762;
    let t17313 = t9217 * t16687;
    let t17316 = t2102 * t16694;
    (t17296, t17299, t17302, t17305, t17310, t17313, t17316)
}
