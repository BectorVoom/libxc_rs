//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1497/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1497<F: Float>(t116913: F, t116915: F, t116917: F, t116927: F, t116930: F, t116932: F, t116934: F, t116936: F, t116968: F, t116969: F, t116971: F, t116995: F) -> F {
    let t117572 = F::cast_from(2.0_f64) * t116913 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t116915 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t116917 + F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t116927 - F::cast_from(110.0_f64) / F::cast_from(27.0_f64) * t116930 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t116932 - F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t116934 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t116936 + t116968 + F::cast_from(110.0_f64) / F::cast_from(27.0_f64) * t116969 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t116971 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t116995;
    t117572
}
