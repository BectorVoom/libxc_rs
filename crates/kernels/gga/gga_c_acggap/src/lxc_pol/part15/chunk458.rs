//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 458/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk458<F: Float>(t467: F, t814: F, t104: F, t642: F, t310: F, t635: F, t315: F, t633: F) -> (F, F, F, F) {
    let t2166 = t814 * t467;
    let t2170 = t104 * t642;
    let t2175 = F::cast_from(0.65854491829355115987e0_f64) * t310 * t635;
    let t2176 = t315 * t633;
    (t2166, t2170, t2175, t2176)
}
