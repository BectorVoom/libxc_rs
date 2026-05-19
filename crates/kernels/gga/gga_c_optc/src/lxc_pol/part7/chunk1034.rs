//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1034/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1034<F: Float>(t1824: F, t1827: F, t1788: F, t1791: F, t13: F, t22494: F, t1792: F, t6452: F, t1755: F, t6454: F, t1863: F, t1866: F) -> (F, F, F, F, F, F, F) {
    let t22563 = t1824 * t1824;
    let t22566 = t1827 * t1827;
    let t22571 = t1788 * t1788;
    let t22574 = t1791 * t1791;
    let t22578 = F::cast_from(0.24954977986735470917e5_f64) * t13 / t22571 * t22494 / t22574;
    let t22581 = F::cast_from(0.57894567559743977359e3_f64) * t6452 * t22494 * t1792;
    let t22593 = F::cast_from(0.620700176468474021e4_f64) * t13 / t1788 / t1755 * t22494 * t6454;
    let t22597 = t1863 * t1863;
    let t22598 = F::new(1.0) / t22597;
    let t22600 = t1866 * t1866;
    (t22563, t22566, t22578, t22581, t22593, t22598, t22600)
}
