//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1106/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1106<F: Float>(t1410: F, t407: F, t1931: F, t980: F, t377: F, t6552: F, t1160: F, t4210: F, t6465: F, t4180: F, t6483: F, t3088: F, t4183: F, t6482: F) -> (F, F, F, F, F, F) {
    let t19834 = t407 * t1410;
    let t19838 = t980 * t1931;
    let t19840 = t377 * t6552;
    let t19843 = t1160 * t6465 * t4210;
    let t19845 = t4180 * t6483;
    let t19854 = t3088 * t6482 * t4183;
    (t19834, t19838, t19840, t19843, t19845, t19854)
}
