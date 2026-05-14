//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1359/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1359<F: Float>(t104624: F, t104627: F, t104632: F, t105329: F, t105336: F, t105338: F, t105952: F, t106167: F, t1349: F, t160: F, t165: F, t26526: F, t26561: F, t26581: F, t26771: F, t28: F, t525: F, t5766: F, t5781: F, t609: F, t9439: F, t94993: F) -> (F,) {
    let t106173 = t94993 / 9.0 - 4.0 * t104624 - t104627 - 2.0 / 3.0 * t26581 * t5781 - t104632 + t5766 * t26561 / 3.0 + t5766 * t26771 / 3.0 + t1349 * t28 * t525 * t105329 * t165 / 6.0 + 4.0 * t105336 + 8.0 * t105338 - 2.0 * t105952 + 2.0 * t106167 * t160 - 24.0 * t9439 * t26526 * t609;
    (t106173,)
}
