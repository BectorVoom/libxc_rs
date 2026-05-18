//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 777/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk777<F: Float>(t12635: F, t587: F, t12355: F, t643: F, t642: F, t639: F, t12350: F, t5401: F, t5400: F, t2601: F, t3553: F, t1621: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12637 = F::new(4.0) / F::new(15.0) * t587 * t12635;
    let t12638 = t643 * t12355;
    let t12639 = t642 * t12638;
    let t12641 = F::new(4.0) / F::new(45.0) * t639 * t12639;
    let t12642 = t5401 * t12350;
    let t12643 = t5400 * t12642;
    let t12645 = F::new(32.0) / F::new(81.0) * t639 * t12643;
    let t12646 = t2601 * t3553;
    let t12647 = t1621 * t12646;
    (t12637, t12638, t12639, t12641, t12642, t12643, t12645, t12646, t12647)
}
