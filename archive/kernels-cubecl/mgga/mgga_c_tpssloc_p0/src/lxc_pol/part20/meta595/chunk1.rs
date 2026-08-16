//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2175/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2175<F: Float>(t1174: F, t1186: F, t2402: F, t11498: F, t135: F, t457: F, t625: F, t221: F, t456: F, t461: F, t11517: F, t11539: F) -> (F, F, F, F, F) {
    let t44478 = t1174 * t2402 * t1186;
    let t44481 = t1174 * t135 * t11498;
    let t44483 = t625 * t457;
    let t44487 = F::cast_from(0.82304526748971193413e-3_f64) * t456 * t221 * t44483 * t461;
    let t44499 = t1174 * t11539 * t11517;
    (t44478, t44481, t44483, t44487, t44499)
}
