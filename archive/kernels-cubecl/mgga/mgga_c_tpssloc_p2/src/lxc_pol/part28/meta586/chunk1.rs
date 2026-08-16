//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1878/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1878<F: Float>(t12971: F, t6552: F, t6637: F, t6638: F, t22893: F, t23164: F, t25312: F, t1888: F, t232: F, t47425: F, t6646: F, t25038: F, t25248: F, t776: F, t87130: F) -> (F, F, F, F) {
    let t87676 = t6552 * t6637 * t6638 * t12971;
    let t87679 = t23164 * t22893 * t25312;
    let t87692 = t1888 * t6646 * t47425 * t232;
    let t87699 = t25038 * t25248 * t87130 * t776;
    (t87676, t87679, t87692, t87699)
}
