//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1248/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1248<F: Float>(t23405: F, t30169: F, t30285: F, t1017: F, t104213: F, t104217: F, t104220: F, t104225: F, t118681: F, t119175: F, t1349: F, t165: F, t26815: F, t26817: F, t28: F, t30108: F, t3588: F, t525: F, t5766: F, t5778: F, t94191: F, t94198: F, t94201: F) -> (F,) {
    let t119181 = t23405 * t30169;
    let t119183 = t23405 * t30285;
    let t119185 = t104213 + t104217 + 2.0 / 27.0 * t94191 - t104220 + 2.0 / 27.0 * t94198 - 2.0 / 3.0 * t1349 * t28 * t5778 * t3588 * t1017 - t118681 / 9.0 - t104225 + 2.0 * t26817 * t26815 - 4.0 / 27.0 * t94201 + t5766 * t30108 / 6.0 + t1349 * t28 * t525 * t119175 * t165 / 6.0 - t119181 / 27.0 - 2.0 / 27.0 * t119183;
    (t119185,)
}
