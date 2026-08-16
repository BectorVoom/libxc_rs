//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1182/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1182<F: Float>(t3733: F, t491: F, t27388: F, t4142: F, t27431: F, t27368: F, t61287: F) -> (F, F, F, F) {
    let t94216 = t3733 * t491;
    let t94223 = t4142 * t27388;
    let t94225 = t4142 * t27431;
    let t94227 = t27368 * t61287;
    (t94216, t94223, t94225, t94227)
}
