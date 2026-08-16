//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 709/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk709<F: Float>(t6756: F, t743: F, t1911: F, t1916: F, t188: F, t1972: F, t712: F, t171: F, t1974: F, t2045: F, t592: F, t2042: F, t559: F) -> (F, F, F, F, F, F, F) {
    let t6757 = t743 * t6756;
    let t6760 = t1916 * t1911;
    let t6761 = t188 * t6760;
    let t6763 = t1972 * t712;
    let t6766 = F::cast_from(1.0_f64) / t1974 / t171;
    let t6770 = t2045 * t592;
    let t6771 = F::cast_from(36.0_f64) * t6770;
    let t6772 = t2042 * t559;
    (t6757, t6760, t6761, t6763, t6766, t6771, t6772)
}
