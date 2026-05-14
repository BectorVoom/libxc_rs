//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 950/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk950<F: Float>(t27489: F, t589: F, t1505: F, t7938: F, t1555: F, t2247: F, t4188: F, t4190: F, t4310: F, t7940: F, t12335: F, t2253: F, t12338: F, t7943: F, t4184: F, t7962: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27490 = t27489 * t589;
    let t27491 = t7938 * t1505;
    let t27493 = 2.0 * t27491 * t1555;
    let t27494 = t2247 * t4188;
    let t27496 = 2.0 * t27494 * t4190;
    let t27497 = t7940 * t4310;
    let t27498 = t12335 * t2253;
    let t27500 = 4.0 * t12338 * t7943;
    let t27502 = 2.0 * t4184 * t7962;
    (t27490, t27491, t27493, t27494, t27496, t27497, t27498, t27500, t27502)
}
