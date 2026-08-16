//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 532/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk532(t4162: f64, t5644: f64, t4160: f64, t1497: f64, t1650: f64, t4171: f64) -> (f64, f64, f64) {
    let t5645 = t4162 * t5644;
    let t5646 = t4160 * t5645;
    let t5648 = t1650 * t1497;
    let t5649 = t4171 * t5648;
    (t5645, t5646, t5649)
}
