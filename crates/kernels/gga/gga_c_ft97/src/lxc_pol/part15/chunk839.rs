//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 839/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk839<F: Float>(t1882: F, t20716: F, t20931: F, t8392: F, t160: F, t20655: F, t20862: F, t20897: F, t9438: F, t20744: F, t20974: F, t20630: F, t549: F, t20607: F, t39922: F, t19977: F, t422: F, t528: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76458 = t1882 * t20716;
    let t76470 = t8392 * t20931;
    let t76567 = t160 * t20655;
    let t76607 = t8392 * t20862;
    let t76618 = t9438 * t20897;
    let t76623 = t8392 * t20744;
    let t76777 = t1882 * t20974;
    let t76876 = t549 * t20630;
    let t76899 = t39922 * t20607;
    let t76914 = t422 * t19977 * t528;
    (t76458, t76470, t76567, t76607, t76618, t76623, t76777, t76876, t76899, t76914)
}
