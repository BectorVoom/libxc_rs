//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 924/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk924<F: Float>(t5179: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5154: F, t5170: F, t7030: F, t7031: F, t7032: F, t7034: F, t7037: F, t7039: F, t7041: F, t7042: F) -> (F, F) {
    let t7043 = F::cast_from(24.0_f64) * t5179;
    let t7044 = t7030 - t5154 - t7031 - t7032 + t4996 + t5005 - t5011 + t5170 - t7034 - t7037 - t7039 + t7041 + t5019 - t5022 - t7042 - t7043;
    (t7043, t7044)
}
