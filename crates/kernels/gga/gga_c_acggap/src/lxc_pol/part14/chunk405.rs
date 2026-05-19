//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 405/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk405<F: Float>(t104: F, t624: F, t301: F, t469: F, t310: F, t611: F, t315: F, t609: F) -> (F, F, F, F) {
    let t1953 = t104 * t624;
    let t1954 = t469 * t301;
    let t1959 = F::cast_from(0.65854491829355115987e0_f64) * t310 * t611;
    let t1960 = t315 * t609;
    (t1953, t1954, t1959, t1960)
}
