//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 604/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk604<F: Float>(t1039: F, t3775: F, t1029: F, t997: F, t1032: F, t993: F, t1205: F, t3266: F, t386: F, t388: F, t384: F, t1103: F) -> (F, F, F, F, F, F, F) {
    let t3777 = F::cast_from(0.12862205435420921092e-2_f64) * t3775 * t1039;
    let t3778 = t997 * t1029;
    let t3782 = F::cast_from(0.30011812682648815881e-2_f64) * t1032 * t993;
    let t3783 = t997 * t1205;
    let t3786 = t386 * t3266 * t388;
    let t3787 = t384 * t3786;
    let t3793 = t1032 * t1103;
    (t3777, t3778, t3782, t3783, t3786, t3787, t3793)
}
