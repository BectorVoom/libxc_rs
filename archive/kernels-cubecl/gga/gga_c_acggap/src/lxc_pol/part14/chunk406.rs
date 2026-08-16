//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 406/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk406<F: Float>(t1960: F, t323: F, t326: F, t38: F, t56: F, t593: F) -> (F, F, F, F) {
    let t1962 = F::cast_from(0.65854491829355115987e0_f64) * t1960 * t323;
    let t1963 = t38 * t326;
    let t1964 = F::cast_from(1.0_f64) / t1963;
    let t1965 = t1964 * t56;
    let t1966 = t593 * t1965;
    (t1962, t1963, t1964, t1966)
}
