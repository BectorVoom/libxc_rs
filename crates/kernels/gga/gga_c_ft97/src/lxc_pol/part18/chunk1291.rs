//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1291/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1291<F: Float>(t1013: F, t22711: F, t94460: F, t12550: F, t1701: F, t5546: F, t12411: F, t135: F, t5820: F, t3379: F, t53: F, t5591: F, t72: F, t22652: F, t2035: F, t23728: F) -> (F, F, F, F, F, F) {
    let t104797 = t94460 * t22711 * t1013;
    let t104813 = t1701 * t5546 * t12550;
    let t104819 = t12411 * t135 * t5820;
    let t104824 = t5591 * t72 * t3379 * t53;
    let t104834 = t1701 * t22652 * t3379;
    let t104838 = t2035 * t23728 * t1013;
    (t104797, t104813, t104819, t104824, t104834, t104838)
}
