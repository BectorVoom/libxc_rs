//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1395/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1395<F: Float>(t101791: F, t101807: F, t101823: F, t103953: F, t29676: F, t29679: F, t8: F, t93848: F, t93849: F, t93852: F, t99792: F, t99793: F, t99794: F, t99795: F, t99796: F) -> F {
    let t103957 = t29676 + t8 * (t101791 + t101807 + t101823 + t103953) + t93848 - t99792 - t99793 - t93849 - t99794 - t29679 + t99795 + t99796 + t93852;
    t103957
}
