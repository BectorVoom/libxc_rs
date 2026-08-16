//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1261/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1261<F: Float>(t111: F, t27370: F, t112: F, t27907: F, t8110: F, t7684: F, t8944: F, t1808: F, t254: F, t1307: F, t1842: F, t1835: F) -> (F, F, F, F, F, F, F) {
    let t96238 = t27370 * t111;
    let t96311 = t27907 * t112;
    let t96334 = t8110 * t111;
    let t96797 = t7684 * t8944;
    let t97626 = t1808 * t254;
    let t97721 = t1842 * t1307;
    let t97740 = t1835 * t254;
    (t96238, t96311, t96334, t96797, t97626, t97721, t97740)
}
