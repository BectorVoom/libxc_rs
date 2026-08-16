//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1848/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1848<F: Float>(t4034: F, t7468: F, t1266: F, t7467: F, t652: F, t6876: F, t7756: F, t645: F, t72: F, t7431: F, t1437: F, t1864: F) -> (F, F, F, F, F, F) {
    let t26002 = F::cast_from(2.0_f64) * t4034 * t7468;
    let t26003 = t1266 * t7467;
    let t26005 = F::cast_from(2.0_f64) * t652 * t26003;
    let t26006 = t6876 * t7756;
    let t26009 = t72 * t7431 * t645;
    let t26012 = t1864 * t1437;
    (t26002, t26003, t26005, t26006, t26009, t26012)
}
