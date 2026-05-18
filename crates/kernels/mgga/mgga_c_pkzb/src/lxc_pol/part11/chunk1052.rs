//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1052/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1052<F: Float>(t158: F, t165: F, t5387: F, t1721: F, t1511: F, t5331: F, t1613: F, t4952: F, t542: F, t555: F, t148: F, t1515: F, t1518: F, t204: F) -> (F, F, F, F, F) {
    let t16421 = t158 / t5387 / t165;
    let t16425 = t1721 * t1721;
    let t16476 = t1511 * t5331;
    let t16481 = F::new(0.46785788981077169656e1) * t555 * t1613 * t4952 * t542;
    let t16486 = F::new(0.28493333333333333333e0) * t204 * t148 * t1515 * t1518;
    (t16421, t16425, t16476, t16481, t16486)
}
