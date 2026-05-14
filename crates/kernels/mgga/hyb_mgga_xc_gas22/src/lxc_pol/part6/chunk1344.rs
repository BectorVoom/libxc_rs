//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1344/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1344<F: Float>(t10851: F, t10852: F, t11187: F, t11188: F, t11190: F, t11259: F, t11260: F, t11618: F, t31692: F, t4: F, t8222: F, t8950: F, t9416: F, t9417: F, t9814: F, t27002: F) -> (F,) {
    let t31707 = t4 * t31692 + 2.0 * t10851 + 2.0 * t10852 + 2.0 * t11187 + 4.0 * t11188 + 2.0 * t11190 + 2.0 * t11259 + 2.0 * t11260 + 2.0 * t11618 + 2.0 * t8222 + 2.0 * t8950 + 2.0 * t9416 + 4.0 * t9417 + 2.0 * t9814;
    let tv4rho42 = t27002 + t31707;
    (tv4rho42,)
}
