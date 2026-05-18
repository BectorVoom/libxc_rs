//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1207/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1207<F: Float>(t1181: F, t1350: F, t1567: F, t3361: F, t4396: F, t5936: F, t1426: F, t1713: F, t175: F, t384: F, t879: F, t13298: F, t13364: F, t21143: F, t525: F) -> (F, F, F, F) {
    let t22021 = t3361 * t1181 * t1567 * t1350;
    let t22023 = t4396 * t5936;
    let t22032 = t384 * t1426 * t175 * t1713 * t879;
    let t22038 = t13298 * t13364 * t525 * t21143;
    (t22021, t22023, t22032, t22038)
}
