//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 981/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk981<F: Float>(t47243: F, t6066: F, t6111: F, t10914: F, t10915: F, t1445: F, t1998: F, t47270: F, t701: F, t326: F, t825: F, t2684: F, t7585: F) -> (F, F, F, F, F) {
    let t47549 = t6111 * t6066 * t47243;
    let t47552 = t10914 * t10915 * t47243;
    let t47562 = F::cast_from(0.23005755572352449806e1_f64) * t1998 * t1445 * t47270 * t701;
    let t47564 = t825 * t326 * t47243;
    let t47567 = t2684 * t7585 * t47243;
    (t47549, t47552, t47562, t47564, t47567)
}
