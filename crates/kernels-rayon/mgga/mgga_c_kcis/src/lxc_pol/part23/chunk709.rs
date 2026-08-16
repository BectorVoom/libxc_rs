//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 709/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk709(t1506: f64, t8207: f64, t2256: f64, t6193: f64, t2109: f64, t7969: f64, t6176: f64) -> (f64, f64, f64, f64) {
    let t8208 = t1506 * t8207;
    let t8209 = t6193 * t2256;
    let t8212 = t7969 * t2109;
    let t8213 = t6176 * t8212;
    (t8208, t8209, t8212, t8213)
}
