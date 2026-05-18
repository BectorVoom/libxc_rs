//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1074/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1074<F: Float>(t41576: F, t8571: F, t236: F, t618: F, t1981: F, t3134: F, t8512: F, t10100: F, t3352: F, t495: F, t8517: F, t1756: F, t2084: F, t2145: F, t27: F) -> (F, F, F, F) {
    let t47602 = t8571 * t41576;
    let t47604 = t236 * t618;
    let t47607 = t8512 * t1981 * t3134 * t47604;
    let t47612 = t8517 * t3352 * t236 * t10100 * t495;
    let t47616 = t2145 * t27 * t2084 * t1756;
    (t47602, t47607, t47612, t47616)
}
