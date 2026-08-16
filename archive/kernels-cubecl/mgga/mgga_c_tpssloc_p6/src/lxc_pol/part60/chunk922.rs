//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 922/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk922<F: Float>(t240: F, t241: F, t2627: F, t812: F, t814: F, t835: F, t23094: F, t30703: F, t23103: F, t794: F, t8339: F, t226: F, t235: F, t2690: F, t8344: F) -> (F, F, F, F, F) {
    let t112792 = t812 * t2627 * t240 * t241;
    let t112802 = t812 * t814 * t835 * t241;
    let t112834 = t23094 * t30703;
    let t112840 = t23103 * t794 * t8339;
    let t112850 = t226 * t235 * t2690 * t8344;
    (t112792, t112802, t112834, t112840, t112850)
}
