//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1035/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1035<F: Float>(t300: F, t3638: F, t779: F, t2104: F, t54: F, t9257: F, t9259: F, t1123: F, t5633: F, t21787: F, t2922: F, t9292: F, t5974: F, t9273: F, t2899: F, t9310: F) -> (F, F, F, F, F, F) {
    let t25337 = t300 * t779 * t3638;
    let t25351 = t2104 * t54 * t9257 * t9259;
    let t25357 = t300 * t5633 * t1123;
    let t25434 = t2922 * t21787 * t9292;
    let t25448 = t2922 * t5974 * t9273;
    let t25453 = t2899 * t5974 * t9310;
    (t25337, t25351, t25357, t25434, t25448, t25453)
}
