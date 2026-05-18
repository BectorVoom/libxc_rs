//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 878/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk878<F: Float>(t1098: F, t3228: F, t1108: F, t3244: F, t1086: F, t1113: F, t1032: F, t3348: F, t3328: F, t377: F, t947: F, t1036: F, t1095: F, t1131: F, t398: F, t864: F) -> (F, F, F, F, F, F, F, F) {
    let t12838 = t3228 * t1098;
    let t12840 = t3244 * t1108;
    let t12842 = t3228 * t1086;
    let t12844 = t3244 * t1113;
    let t12848 = t1032 * t3348;
    let t12854 = t377 * t3328;
    let t12855 = t12854 * t947;
    let t12862 = t1036 * t398 * t1095 * t1131 * t864;
    (t12838, t12840, t12842, t12844, t12848, t12854, t12855, t12862)
}
