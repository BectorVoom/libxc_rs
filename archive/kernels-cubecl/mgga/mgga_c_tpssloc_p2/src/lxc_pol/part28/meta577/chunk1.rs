//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1861/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1861<F: Float>(t12971: F, t1894: F, t236: F, t6591: F, t23046: F, t4184: F, t812: F, t836: F, t13080: F, t23146: F, t242: F, t81816: F) -> (F, F, F, F) {
    let t87359 = t6591 * t1894 * t236 * t12971;
    let t87363 = t812 * t23046 * t836 * t4184;
    let t87365 = t23146 * t13080;
    let t87368 = t812 * t81816 * t242;
    (t87359, t87363, t87365, t87368)
}
