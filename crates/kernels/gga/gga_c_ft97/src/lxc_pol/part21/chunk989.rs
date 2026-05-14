//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 989/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk989<F: Float>(t147: F, t30166: F, t30564: F, t184: F, t1080: F, t1395: F, t21: F, t27440: F, t4431: F, t4890: F, t4895: F, t4898: F, t5: F, t5985: F, t6732: F, t920: F, t7240: F, t81: F) -> (F, F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t30565 = t30166 + t30564;
    let t30566 = t30565 * t184;
    let t30585 = piecewise3(t148, 0.0, t5 * t30566 * t21 / 4.0 + t27440 * t1080 / 2.0 + t5 * t6732 * t920 / 2.0 + t5985 * t4890 / 4.0 + t5985 * t4895 / 4.0 + t5985 * t4898 / 2.0 + t5 * t1395 * t4431 / 4.0);
    let t32075 = 1.0 / t7240 / t81;
    (t30565, t30566, t30585, t32075)
}
