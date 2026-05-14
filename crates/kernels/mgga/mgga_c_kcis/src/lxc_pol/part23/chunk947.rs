//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 947/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk947<F: Float>(t1386: F, t1444: F, t2642: F, t5709: F, t3964: F, t491: F, t990: F) -> (F, F, F, F) {
    let t27453 = t1386 * t1444;
    let t27454 = t27453 * t2642;
    let t27455 = t5709 * t27454;
    let t27459 = t3964 * t491 * t990;
    (t27453, t27454, t27455, t27459)
}
