//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 863/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk863<F: Float>(t9947: F, t9957: F, t871: F, t3801: F, t881: F, t6090: F, t6127: F, t7955: F, t8038: F, t9782: F, t9797: F, t378: F, t3734: F, t832: F, t853: F, t1185: F, t8214: F) -> (F, F, F, F, F, F, F, F) {
    let t9958 = t9947 + t9957;
    let t9959 = t9958 * t871;
    let t9964 = t3801 * t881;
    let t9973 = -t6127 + 0.12361111111111111111e-1 * t6090 + 0.24722222222222222223e-1 * t7955 - t8038 - 0.92708333333333333333e-2 * t9782 + 0.278125e-1 * t9797;
    let t9974 = t9973 * t378;
    let t9976 = t3734 * t832;
    let t9978 = 1.0 * t9976 * t853;
    let t9980 = 2.0 * t8214 * t1185;
    (t9958, t9959, t9964, t9973, t9974, t9976, t9978, t9980)
}
