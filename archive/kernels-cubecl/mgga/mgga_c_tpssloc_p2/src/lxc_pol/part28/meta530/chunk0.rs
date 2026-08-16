//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1784/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1784<F: Float>(t22700: F, t6914: F, t22699: F, t22704: F, t22705: F, t22741: F, t22696: F, t3879: F, t552: F, t22747: F, t22893: F, t80681: F) -> (F, F, F, F, F, F) {
    let t81099 = t6914 * t22700;
    let t81115 = t22704 * t22705 * t22699;
    let t81125 = t22704 * t22705 * t22741;
    let t81127 = t6914 * t22696;
    let t81129 = t552 * t3879;
    let t81140 = t80681 * t22893 * t22747;
    (t81099, t81115, t81125, t81127, t81129, t81140)
}
