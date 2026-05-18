//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 949/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk949<F: Float>(t47964: F, t9287: F, t2365: F, t38272: F, t7025: F, t38770: F, t901: F, t38486: F, t13792: F, t4379: F, t12000: F, t1429: F, t2366: F) -> (F, F, F, F, F, F) {
    let t47965 = t47964 * t9287;
    let t47968 = t7025 * t2365 * t38272;
    let t47976 = t38770 * t901;
    let t47978 = t38486 * t901;
    let t47980 = t4379 * t13792;
    let t47984 = t1429 * t2365 * t2366 * t12000;
    (t47965, t47968, t47976, t47978, t47980, t47984)
}
