//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 714/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk714<F: Float>(t1900: F, t227: F, t5519: F, t1937: F, t690: F, t1936: F, t244: F, t239: F) -> (F, F, F, F, F) {
    let t5804 = 1.0 / t1900 / t227;
    let t5812 = 0.53272592592592592592e-1 * t5519;
    let t5825 = t690 * t1937;
    let t5829 = 1.0 / t1936 / t244;
    let t5830 = t239 * t5829;
    (t5804, t5812, t5825, t5829, t5830)
}
