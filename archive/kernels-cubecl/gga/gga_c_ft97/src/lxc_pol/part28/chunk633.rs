//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 633/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk633<F: Float>(t26077: F, t26089: F, t26102: F, t26111: F, t488: F, t1307: F, t984: F, t1564: F, t379: F, t22873: F, t6421: F, t28: F) -> (F, F, F, F) {
    let t26113 = t26077 + t26089 + t26102 + t26111;
    let t26114 = t488 * t26113;
    let t26117 = t1307 * t984;
    let t26119 = t1564 * t26117 * t379;
    let t26124 = t22873 * t6421;
    let t26125 = t28 * t26124;
    (t26113, t26114, t26119, t26125)
}
