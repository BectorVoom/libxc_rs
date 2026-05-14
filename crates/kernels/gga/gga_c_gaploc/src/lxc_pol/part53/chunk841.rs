//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 841/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk841<F: Float>(t12218: F, t1445: F, t2087: F, t2530: F, t13862: F, t2197: F, t47220: F, t833: F, t38974: F, t813: F, t935: F, t47243: F, t6066: F, t6111: F, t10914: F, t10915: F) -> (F, F, F, F, F, F) {
    let t47535 = t2087 * t1445 * t12218 * t2530;
    let t47537 = t2197 * t13862;
    let t47540 = t833 * t1445 * t47220;
    let t47544 = t813 * t1445 * t38974 * t935;
    let t47549 = t6111 * t6066 * t47243;
    let t47552 = t10914 * t10915 * t47243;
    (t47535, t47537, t47540, t47544, t47549, t47552)
}
