//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2422/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2422<F: Float>(t6589: F, t67: F, t246: F, t2784: F, t2841: F, t22715: F, t268: F, t271: F) -> (F, F, F, F) {
    let t41466 = t6589 * t67;
    let t41467 = t41466 * t246;
    let t41623 = t2784 * t2841;
    let t41654 = t268 * t22715 * t271;
    (t41466, t41467, t41623, t41654)
}
