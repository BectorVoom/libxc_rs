//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1325/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1325<F: Float>(t22986: F, t28267: F, t86873: F, t20800: F, t6552: F, t6637: F, t6638: F, t1888: F, t20873: F, t6646: F, t1510: F, t25038: F, t98336: F) -> (F, F, F, F) {
    let t105519 = t22986 * t86873 * t28267;
    let t105531 = t6552 * t6637 * t6638 * t20800;
    let t105543 = t1888 * t6646 * t20873;
    let t105547 = t25038 * t6646 * t98336 * t1510;
    (t105519, t105531, t105543, t105547)
}
