//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 280/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk280<F: Float>(t944: F, t945: F, t397: F, t937: F, t943: F) -> (F, F) {
    let t946 = t944 * t945;
    let t951 = F::cast_from(0.65854491829355115987e0_f64) * t943 * t946 + F::cast_from(0.65854491829355115987e0_f64) * t397 * t937;
    (t946, t951)
}
