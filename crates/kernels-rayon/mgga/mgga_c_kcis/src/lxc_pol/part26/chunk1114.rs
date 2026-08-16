//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1114/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1114(t2069: f64, t7962: f64, t4189: f64, t2253: f64, t6048: f64, t4184: f64, t8207: f64, t1555: f64, t1468: f64, t6034: f64, t2055: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28573 = t7962 * t2069;
    let t28575 = 2.0_f64 * t4189 * t28573;
    let t28576 = t2253 * t6048;
    let t28578 = 2.0_f64 * t4189 * t28576;
    let t28579 = t4184 * t8207;
    let t28580 = t8207 * t1555;
    let t28582 = 2.0_f64 * t4189 * t28580;
    let t28583 = t1468 * t6034;
    let t28585 = t3734 * t2055;
    (t28573, t28575, t28576, t28578, t28579, t28580, t28582, t28583, t28585)
}
