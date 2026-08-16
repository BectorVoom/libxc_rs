//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2548/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2548<F: Float>(t3038: F, t49650: F, t1041: F, t13611: F, t248: F, t3051: F, t14137: F, t3117: F, t10413: F, t10422: F, t14125: F, t10965: F, t4571: F) -> (F, F, F, F, F) {
    let t49771 = t49650 * t3038;
    let t49799 = t1041 * t248 * t3051 * t13611;
    let t49801 = t3117 * t14137;
    let t49808 = t10413 * t10422 * t14125;
    let t49810 = t10965 * t4571;
    (t49771, t49799, t49801, t49808, t49810)
}
