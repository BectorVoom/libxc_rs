//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 48/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk48<F: Float>(t55: F, t58: F, t61: F, t69: F) -> (F, F, F) {
    let t119 = 0.51785e1 * t58 + 0.905775e0 * t55 + 0.1100325e0 * t61 + 0.1241775e0 * t69;
    let t122 = 1.0 + 0.29608749977793437516e2 / t119;
    let t123 = f64::ln(t122);
    (t119, t122, t123)
}
