//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1123/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1123<F: Float>(t236: F, t27150: F, t233: F, t2167: F, t2651: F, t234: F, t2793: F, t2170: F, t7828: F, t911: F, t8027: F, t4527: F) -> (F, F, F, F, F, F, F) {
    let t27151 = t236 * t27150;
    let t27152 = t233 * t27151;
    let t27153 = t27152 / F::cast_from(16.0_f64);
    let t27154 = t2651 * t2167;
    let t27155 = t2793 * t234;
    let t27156 = t27155 * t2170;
    let t27157 = t27156 / F::cast_from(8.0_f64);
    let t27158 = t911 * t7828;
    let t27159 = t27158 / F::cast_from(8.0_f64);
    let t27731 = t911 * t8027;
    let t27733 = t4527 * t2167;
    (t27153, t27154, t27155, t27157, t27159, t27731, t27733)
}
