//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1799/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1799<F: Float>(t849: F, t87340: F, t1516: F, t81763: F, t23083: F, t25094: F, t23046: F, t4184: F, t812: F, t836: F, t242: F, t81816: F) -> (F, F, F, F, F) {
    let t87341 = t87340 * t849;
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87363 = t812 * t23046 * t836 * t4184;
    let t87368 = t812 * t81816 * t242;
    (t87341, t87345, t87347, t87363, t87368)
}
