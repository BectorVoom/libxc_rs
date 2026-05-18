//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1050/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1050<F: Float>(t27803: F, t7703: F, t291: F, t417: F, t1008: F, t13097: F) -> (F, F, F, F) {
    let t27804 = t7703 * t27803;
    let t27806 = t417 * t291;
    let t27807 = t13097 * t1008;
    let t27808 = t27806 * t27807;
    (t27804, t27806, t27807, t27808)
}
