//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1915/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1915<F: Float>(t22892: F, t22893: F, t26384: F, t16018: F, t6637: F, t6888: F, t6968: F, t26388: F, t7733: F, t81186: F, t5318: F, t552: F) -> (F, F, F, F, F) {
    let t90797 = t22892 * t22893 * t26384;
    let t90801 = t6888 * t6637 * t6968 * t16018;
    let t90805 = t22892 * t22893 * t26388;
    let t90807 = t81186 * t7733;
    let t90809 = t552 * t5318;
    (t90797, t90801, t90805, t90807, t90809)
}
