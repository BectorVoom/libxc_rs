//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 842/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk842<F: Float>(t1445: F, t1998: F, t47270: F, t701: F, t326: F, t47243: F, t825: F, t2684: F, t7585: F, t12207: F, t9823: F, t41528: F, t41532: F, t41534: F, t13846: F, t1841: F, t2536: F, t734: F) -> (F, F, F, F, F, F, F, F) {
    let t47562 = 0.23005755572352449806e1 * t1998 * t1445 * t47270 * t701;
    let t47564 = t825 * t326 * t47243;
    let t47567 = t2684 * t7585 * t47243;
    let t47572 = t9823 * t12207;
    let t47574 = 0.38342925953920749677e0 * t41528;
    let t47575 = 0.85206502119823888171e-1 * t41532;
    let t47576 = 0.38342925953920749677e0 * t41534;
    let t47587 = t1841 * t2536 * t13846 * t734;
    (t47562, t47564, t47567, t47572, t47574, t47575, t47576, t47587)
}
