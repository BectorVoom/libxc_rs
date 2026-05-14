//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 325/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk325<F: Float>(t428: F, t5522: F, t397: F, t52: F, t67: F, t11: F, t391: F, t41: F) -> (F, F, F) {
    let t5523 = t5522 * t428;
    let t5530 = t52 * t67 * t397;
    let t5532 = -0.1201569457037037037e0 * t41 * t11 * t391 - 0.59273806478425129877e-2 * t5530;
    (t5523, t5530, t5532)
}
