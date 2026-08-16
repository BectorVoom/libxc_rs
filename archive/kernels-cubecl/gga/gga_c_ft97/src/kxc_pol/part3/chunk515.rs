//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 515/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk515<F: Float>(t3874: F, t4002: F, t258: F, t3951: F, t1137: F, t1173: F, t247: F, t263: F, t3683: F, t3827: F, t3865: F, t3973: F, t3978: F, t3982: F, t719: F, t771: F) -> (F, F, F) {
    let t4003 = t3874 + t4002;
    let t4005 = t3951 * t258;
    let t4011 = -t1137 * t771 - t1173 * t719 - t247 * t4003 - t263 * t3683 - t263 * t3827 + F::cast_from(4.0_f64) * t3865 - F::cast_from(2.0_f64) * t3973 - F::cast_from(2.0_f64) * t3978 - F::cast_from(2.0_f64) * t3982 + F::cast_from(2.0_f64) * t4005;
    (t4003, t4005, t4011)
}
