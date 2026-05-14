//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1028/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1028<F: Float>(t9: F, t92895: F, t1624: F, t1593: F, t420: F, t422: F, t1710: F, t5532: F, t1737: F, t1720: F, t70: F, t1598: F, t1711: F, t25754: F, t32261: F, t22511: F, t22817: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92896 = t9 * t92895;
    let t92897 = t1624 * t92896;
    let t92899 = t420 * t422 * t1593;
    let t92920 = t1710 * t5532;
    let t92957 = t420 * t1737;
    let t93003 = t1720 * t70;
    let t93014 = t1598 * t1711;
    let t93016 = t25754 * t32261;
    let t93046 = t22817 * t22511;
    (t92896, t92897, t92899, t92920, t92957, t93003, t93014, t93016, t93046)
}
