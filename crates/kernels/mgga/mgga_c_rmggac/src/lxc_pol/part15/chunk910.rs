//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 910/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk910<F: Float>(t25441: F, t9948: F, t202: F, t461: F, t6067: F, t674: F, t678: F, t1763: F, t1970: F, t1971: F, t209: F, t476: F, t875: F) -> (F, F, F) {
    let t45226 = t25441 * t9948;
    let t45234 = t6067 * t202 * t461 * t674 * t678;
    let t45240 = t1970 * t1971 * t875 * t1763 * t476 * t209;
    (t45226, t45234, t45240)
}
