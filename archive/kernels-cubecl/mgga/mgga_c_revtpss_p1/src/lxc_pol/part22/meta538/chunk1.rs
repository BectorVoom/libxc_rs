//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2347/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2347<F: Float>(t3704: F, t5293: F, t1121: F, t1214: F, t606: F, t1250: F, t17353: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F) -> (F, F, F, F, F, F, F) {
    let t17509 = F::cast_from(0.15244095330869239812e-2_f64) * t5293 * t3704;
    let t17512 = t1214 * t1121;
    let t17513 = t17512 * t606;
    let t17514 = t1250 * t17513;
    let t17515 = t17353 * t17514;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    (t17509, t17513, t17514, t17515, t17524, t17525, t17528)
}
