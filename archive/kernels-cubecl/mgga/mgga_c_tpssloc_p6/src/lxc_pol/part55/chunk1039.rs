//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1039/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1039<F: Float>(t235: F, t30725: F, t226: F, t30675: F, t30680: F, t30683: F, t30688: F, t30692: F, t30695: F, t808: F, t812: F, t8360: F) -> (F, F) {
    let t30726 = t235 * t30725;
    let t30728 = t226 * t30726 - t30695 * t812 + t808 * t8360 - t30675 - t30680 - t30683 - t30688 + t30692;
    (t30726, t30728)
}
