//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2034/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2034<F: Float>(t1307: F, t16095: F, t686: F, t16094: F, t12214: F, t131: F, t205: F) -> (F, F, F, F) {
    let t16097 = t686 * t16095 * t1307;
    let t16099 = F::cast_from(0.49999999999999999998e-2_f64) * t16094 * t16097;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    (t16097, t16099, t16100, t16101)
}
