//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1155/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1155<F: Float>(t881: F, t9973: F, t2317: F, t3801: F, t2256: F, t3774: F, t2320: F, t9929: F, t2281: F, t9958: F, t862: F, t9888: F) -> (F, F, F, F, F, F) {
    let t27694 = t9973 * t881;
    let t27699 = t3801 * t2317;
    let t27771 = t3774 * t2256;
    let t27795 = t9929 * t2320;
    let t27812 = t9958 * t2281;
    let t27834 = t9888 * t862;
    (t27694, t27699, t27771, t27795, t27812, t27834)
}
