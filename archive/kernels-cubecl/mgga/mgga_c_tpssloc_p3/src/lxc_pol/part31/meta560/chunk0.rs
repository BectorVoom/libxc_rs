//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1789/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1789<F: Float>(t23168: F, t25338: F, t23012: F, t7485: F, t25046: F, t6579: F, t1484: F, t2717: F, t82099: F, t7489: F, t82120: F, t23164: F, t23204: F, t25341: F) -> (F, F, F, F, F, F, F, F) {
    let t86950 = t23168 * t25338;
    let t86955 = t23012 * t7485;
    let t86967 = t6579 * t25046;
    let t86969 = t2717 * t1484;
    let t86983 = F::cast_from(0.52089578783527170489e-1_f64) * t82099;
    let t86991 = t23012 * t7489;
    let t86994 = F::cast_from(0.3289868133696452873e-1_f64) * t82120;
    let t87028 = t23164 * t23204 * t25341;
    (t86950, t86955, t86967, t86969, t86983, t86991, t86994, t87028)
}
