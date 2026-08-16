//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1122/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1122<F: Float>(t23171: F, t23228: F, t7488: F, t23030: F, t25205: F, t1519: F, t212: F, t6554: F, t1649: F, t2752: F, t1410: F, t9239: F) -> (F, F, F, F, F) {
    let t87779 = t23171 * t23228 * t7488;
    let t87898 = t23030 * t25205;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t89992 = t2752 * t1649;
    let t90137 = t9239 * t1410;
    (t87779, t87898, t87915, t89992, t90137)
}
