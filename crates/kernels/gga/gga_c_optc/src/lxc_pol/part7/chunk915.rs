//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 915/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk915<F: Float>(t496: F, t8428: F, t1113: F, t23: F, t3273: F, t4280: F, t24: F, t3086: F, t8414: F, t8: F, t465: F, t8113: F, t19: F, t3126: F, t4356: F, t1027: F, t8446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11597 = t496 * t8428;
    let t11781 = t23 * t1113;
    let t11786 = t3273 * t4280;
    let t11885 = t24 * t3086;
    let t11894 = t496 * t8414;
    let t11899 = t8 * t3086;
    let t11943 = t465 * t8113;
    let t11970 = t19 * t3126;
    let t11971 = t11970 * t4356;
    let t11975 = t8446 * t1027;
    (t11597, t11781, t11786, t11885, t11894, t11899, t11943, t11971, t11975)
}
