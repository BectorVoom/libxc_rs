//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1183/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1183<F: Float>(t135: F, t1634: F, t1009: F, t4882: F, t5137: F, t1508: F, t7035: F, t496: F, t6825: F, t2562: F, t500: F, t2569: F, t184: F, t5418: F, t16388: F, t2583: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19800 = t135 * t1634;
    let t19803 = t4882 * t1009;
    let t19805 = t5137 * t1009;
    let t19822 = t7035 * t1508;
    let t19824 = t496 * t6825;
    let t19843 = 16.0 * t2562 * t500;
    let t19863 = 16.0 * t2569 * t500;
    let t19873 = t184 * t5418;
    let t19909 = t16388 * t2583;
    (t19800, t19803, t19805, t19822, t19824, t19843, t19863, t19873, t19909)
}
