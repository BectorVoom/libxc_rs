//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1155/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1155<F: Float>(t3281: F, t842: F, t877: F, t10755: F, t1882: F, t10662: F, t681: F, t89: F, t309: F, t43833: F, t870: F, t9570: F) -> (F, F, F, F, F, F) {
    let t44318 = t3281 * t842;
    let t44320 = t3281 * t877;
    let t44330 = t1882 * t10755;
    let t44333 = t89 * t681 * t10662;
    let t44335 = t43833 * t309;
    let t44340 = t870 * t9570;
    (t44318, t44320, t44330, t44333, t44335, t44340)
}
