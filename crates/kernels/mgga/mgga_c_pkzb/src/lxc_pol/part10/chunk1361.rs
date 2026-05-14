//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1361/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1361<F: Float>(t3833: F, t6117: F, t3026: F, t218: F, t219: F, t204: F, t648: F, t9795: F) -> (F, F, F, F) {
    let t27253 = 0.11696447245269292414e1 * t6117 * t3833;
    let t27254 = t3026 * t3026;
    let t27256 = t218 * t219 * t27254;
    let t27262 = t204 * t648 * t9795;
    (t27253, t27254, t27256, t27262)
}
