//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 889/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk889<F: Float>(t42799: F, t42802: F, t42803: F, t42804: F, t42806: F, t42808: F, t42811: F, t42814: F, t42815: F, t42816: F, t42817: F, t42821: F, t42822: F, t42823: F, t42824: F, t47001: F, t47003: F, t47011: F) -> (F,) {
    let t50966 = -t42799 - t42802 - t42803 + t42804 + t42806 - t42808 - t42811 - t42814 + t42815 + t42816 - t42817 - 0.56910013271352299198e-1 * t47001 - 0.31616674039640166221e-2 * t47003 - t42821 - t42822 - t42823 + t42824 + 0.18970004423784099732e-1 * t47011;
    (t50966,)
}
