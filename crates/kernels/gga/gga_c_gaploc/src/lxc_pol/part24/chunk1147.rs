//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1147/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1147<F: Float>(t30820: F, t6895: F, t888: F, t9263: F, t1538: F, t20073: F, t6583: F, t883: F, t4389: F, t899: F, t1415: F, t6490: F, t913: F) -> (F, F, F, F, F) {
    let t30821 = F::new(0.38342925953920749676e0) * t30820;
    let t30823 = t9263 * t888 * t6895;
    let t30824 = F::new(0.76685851907841499352e0) * t30823;
    let t30827 = t6583 * t1538 * t883 * t20073;
    let t30828 = F::new(0.38342925953920749676e0) * t30827;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    let t30833 = F::new(0.11916829983950142223e0) * t30830 * t913 * t6490;
    (t30821, t30824, t30828, t30830, t30833)
}
