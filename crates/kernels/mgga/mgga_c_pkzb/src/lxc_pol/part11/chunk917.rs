//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 917/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk917<F: Float>(t9947: F, t9957: F, t871: F, t3801: F, t881: F, t6090: F, t6127: F, t7955: F, t8038: F, t9782: F, t9797: F, t378: F) -> (F, F, F, F, F) {
    let t9958 = t9947 + t9957;
    let t9959 = t9958 * t871;
    let t9964 = t3801 * t881;
    let t9973 = -t6127 + F::cast_from(0.12361111111111111111e-1_f64) * t6090 + F::cast_from(0.24722222222222222223e-1_f64) * t7955 - t8038 - F::cast_from(0.92708333333333333333e-2_f64) * t9782 + F::new(0.278125e-1) * t9797;
    let t9974 = t9973 * t378;
    (t9958, t9959, t9964, t9973, t9974)
}
