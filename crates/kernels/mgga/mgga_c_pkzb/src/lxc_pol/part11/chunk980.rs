//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 980/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk980<F: Float>(t1020: F, t10556: F, t1058: F, t10592: F, t10593: F, t10594: F, t10731: F, t135: F, t144: F, t1535: F, t2536: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5178: F, t5186: F, t560: F, t639: F, t8751: F, t9112: F, t9121: F) -> F {
    let t10747 = t10731 * t135 * t144 * t639 + F::new(9.0) * t1020 * t1535 * t9112 - F::new(9.0) * t1020 * t1535 * t9121 + F::new(3.0) * t10556 * t135 * t560 - F::new(3.0) * t1058 * t2536 * t8751 - t10592 - t10593 + t10594 + t4996 + t5005 - t5011 + t5019 - t5022 + t5178 + t5186;
    t10747
}
