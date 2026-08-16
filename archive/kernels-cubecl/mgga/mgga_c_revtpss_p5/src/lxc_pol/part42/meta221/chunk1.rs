//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 860/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk860<F: Float>(t108: F, t5911: F, t105: F, t109: F, t1507: F, t1510: F, t5896: F, t5899: F, t5902: F, t5908: F, t97: F) -> F {
    let t5912 = t108 * t5911;
    let t5915 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t97 * t5896 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t5899 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t5902 * t109 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1507 * t1510 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t5908 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t5912;
    t5915
}
