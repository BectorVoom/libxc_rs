//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 733/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk733<F: Float>(t3103: F, t432: F, t110: F, t1871: F, t1755: F, t942: F, t1825: F, t3271: F, t452: F, t1882: F, t3263: F, t3240: F) -> (F, F, F, F, F) {
    let t11520 = t3103 * t432;
    let t11522 = t1871 * t110 * t11520;
    let t11525 = t942 * t1755;
    let t11527 = t1871 * t110 * t11525;
    let t11531 = t452 * t1825 * t3271;
    let t11535 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3263;
    let t11537 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3240;
    (t11522, t11527, t11531, t11535, t11537)
}
