//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 413/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk413<F: Float>(t143: F, t160: F, t4790: F, t2149: F, t3318: F, t3335: F, t4654: F, t4658: F, t4662: F, t4666: F, t4671: F, t4717: F, t4755: F, t4780: F) -> (F, F) {
    let t4792 = t143 * t4790 * t160;
    let t4805 = -t4755 / 4.0 + t4780 / 2.0 + t2149 + 2.0 / 9.0 * t3318 + 2.0 / 3.0 * t3335 - 2.0 / 9.0 * t4654 + 2.0 / 3.0 * t4658 + 2.0 / 3.0 * t4662 - t4666 / 3.0 + 2.0 * t4671 - t4717;
    (t4792, t4805)
}
