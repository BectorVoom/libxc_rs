//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 717/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk717<F: Float>(t5519: F, t5557: F, t1936: F, t693: F, t239: F) -> (F, F, F, F) {
    let t5852 = 0.16068111111111111111e1 * t5519;
    let t5859 = 0.46308888888888888888e0 * t5557;
    let t5870 = 1.0 / t1936 / t693;
    let t5871 = t239 * t5870;
    (t5852, t5859, t5870, t5871)
}
