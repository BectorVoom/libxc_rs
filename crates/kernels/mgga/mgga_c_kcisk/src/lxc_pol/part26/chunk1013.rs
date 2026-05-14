//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1013/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1013<F: Float>(t27089: F, t4230: F, t26796: F, t4204: F, t4203: F, t25301: F, t6369: F, t6368: F, t6332: F, t6331: F, t26987: F, t4231: F, t26411: F, t6317: F, t6316: F, t21321: F, t6357: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27090 = t4230 * t27089;
    let t27092 = t4204 * t26796;
    let t27093 = t4203 * t27092;
    let t27095 = t6369 * t25301;
    let t27096 = t6368 * t27095;
    let t27098 = t6332 * t25301;
    let t27099 = t6331 * t27098;
    let t27101 = t4231 * t26987;
    let t27102 = t4230 * t27101;
    let t27104 = t6317 * t26411;
    let t27105 = t6316 * t27104;
    let t27107 = t21321 * t6357;
    (t27090, t27092, t27093, t27095, t27096, t27098, t27099, t27101, t27102, t27104, t27105, t27107)
}
