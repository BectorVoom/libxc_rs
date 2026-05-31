//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 934/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk934<F: Float>(t1058: F, t5165: F, t2706: F, t639: F, t1535: F, t1673: F, t2536: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5148: F, t5154: F, t5170: F, t568: F, t7023: F, t7025: F, t7030: F, t7031: F, t7032: F, t7034: F, t7037: F, t7039: F, t7041: F, t7042: F) -> (F, F, F) {
    let t7197 = t1058 * t5165;
    let t7201 = t2706 * t639;
    let t7205 = F::cast_from(6.0_f64) * t1535 * t568 * t7201 + F::cast_from(2.0_f64) * t1673 * t2536 * t7197 + t4996 + t5005 - t5011 + t5019 - t5022 - t5148 - t5154 + t5170 + t7023 + t7025 + t7030 - t7031 - t7032 - t7034 - t7037 - t7039 + t7041 - t7042;
    (t7197, t7201, t7205)
}
