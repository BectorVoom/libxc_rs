//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2672/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2672<F: Float>(t10073: F, t14124: F, t5760: F, t9292: F, t213: F, t46518: F, t46520: F, t46526: F, t48080: F, t48082: F, t48085: F, t48090: F, t49161: F, t546: F, t5735: F, t5755: F, t9899: F) -> F {
    let t49167 = t10073 * t14124;
    let t49172 = t9292 * t5760;
    let t49174 = t48080 + t48082 + F::cast_from(0.58544643236296698113e-1_f64) * t48085 + t48090 + t46518 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t49161 - F::cast_from(0.39029762157531132075e-1_f64) * t46520 + F::cast_from(0.33133632253434461091e-3_f64) * t46526 + F::cast_from(0.19514881078765566037e-2_f64) * t49167 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t5735 * t9899 - F::cast_from(0.17073386770573548589e-1_f64) * t49172;
    t49174
}
