//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 707/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk707<F: Float>(t1535: F, t8511: F, t1345: F, t604: F, t1181: F, t7575: F, t2263: F, t4680: F, t2068: F, t1411: F, t599: F, t1983: F, t525: F, t7586: F, t7585: F, t2268: F, t7839: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8512 = t8511 * t1535;
    let t8514 = t604 * t1345;
    let t8515 = t1181 * t8514;
    let t8516 = t7575 * t8515;
    let t8518 = t4680 * t2263;
    let t8519 = t2068 * t8518;
    let t8521 = t599 * t1411;
    let t8522 = t1181 * t8521;
    let t8523 = t2068 * t8522;
    let t8525 = t1983 * t525;
    let t8526 = t7586 * t8525;
    let t8527 = t7585 * t8526;
    let t8529 = t7839 * t2268;
    (t8512, t8514, t8515, t8516, t8518, t8519, t8521, t8522, t8523, t8525, t8526, t8527, t8529)
}
