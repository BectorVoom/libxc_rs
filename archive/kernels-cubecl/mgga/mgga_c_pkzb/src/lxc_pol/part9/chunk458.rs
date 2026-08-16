//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 458/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk458<F: Float>(t1867: F, t665: F, t1830: F, t210: F) -> (F, F, F) {
    let t1868 = t665 * t1867;
    let t1870 = F::cast_from(0.39862222222222222223e0_f64) * t1830;
    let t1873 = F::cast_from(1.0_f64)/F::sqrt(t210);
    (t1868, t1870, t1873)
}
