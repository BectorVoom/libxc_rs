//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1124/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1124<F: Float>(t5752: F, t5781: F, t5780: F, t3738: F, t7033: F, t1394: F, t18431: F, t531: F) -> (F, F, F) {
    let t21014 = t5752 * t5781;
    let t21015 = t5780 * t21014;
    let t21017 = t3738 * t7033;
    let t21018 = t1394 * t21017;
    let t21020 = t531 * t18431;
    (t21015, t21018, t21020)
}
