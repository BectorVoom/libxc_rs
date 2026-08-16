//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1838/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1838<F: Float>(t1519: F, t794: F, t23164: F, t6555: F, t23035: F, t23241: F, t25224: F, t7480: F, t81632: F, t25038: F, t25040: F, t82159: F) -> (F, F, F, F, F) {
    let t86893 = t794 * t1519;
    let t86895 = t23164 * t86893 * t6555;
    let t86901 = t23035 * t25224 * t23241;
    let t86903 = t81632 * t7480;
    let t86909 = t25038 * t82159 * t25040;
    (t86893, t86895, t86901, t86903, t86909)
}
