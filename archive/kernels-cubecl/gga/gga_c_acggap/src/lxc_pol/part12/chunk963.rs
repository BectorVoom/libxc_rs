//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 963/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk963<F: Float>(t1177: F, t31824: F, t13364: F, t31115: F, t31116: F, t30049: F, t7461: F, t1089: F, t1198: F, t2079: F, t2080: F, t1967: F, t7523: F) -> (F, F, F, F, F) {
    let t31825 = t31824 * t1177;
    let t31832 = t31115 * t13364 * t31116;
    let t31839 = t30049 * t7461;
    let t31843 = t2079 * t1089 * t1198 * t2080;
    let t31845 = t1967 * t7523;
    (t31825, t31832, t31839, t31843, t31845)
}
