//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 555/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk555<F: Float>(t4635: F, t4636: F, t124: F, t4599: F, t4595: F, t121: F, t1268: F, t2060: F, t3406: F, t641: F) -> (F, F, F, F) {
    let t4637 = t4635 + t4636;
    let t4643 = t124 * t4599;
    let t4646 = t124 * t4595;
    let t4649 = -0.12897460341341234505e3 * t4637 * t121 * t124 + 0.7738476204804740703e3 * t3406 * t1268 - 0.15476952409609481406e4 * t2060 * t4643 + 0.38692381024023703515e3 * t641 * t4646;
    (t4637, t4643, t4646, t4649)
}
