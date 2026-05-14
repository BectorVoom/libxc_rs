//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 899/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk899<F: Float>(t6837: F, t766: F, t729: F, t762: F, t2469: F, t6861: F, t1168: F, t6061: F, t24658: F, t24673: F, t24690: F, t28236: F, t28239: F, t28243: F, t28248: F, t28252: F, t28257: F, t28260: F, t28264: F, t446: F) -> (F, F, F, F, F, F) {
    let t28267 = t6837 * t766;
    let t28269 = t729 * t762 * t28267;
    let t28273 = t729 * t2469 * t6861;
    let t28276 = t6061 * t1168;
    let t28278 = t729 * t762 * t28276;
    let t28281 = t24658 - t24673 / 27.0 + t24690 / 27.0 + 2.0 / 3.0 * t446 * t28236 + 2.0 / 3.0 * t446 * t28239 + t446 * t28243 / 3.0 + t446 * t28248 / 3.0 + t446 * t28252 / 3.0 + t446 * t28257 / 3.0 + 2.0 / 3.0 * t446 * t28260 + 2.0 / 3.0 * t446 * t28264 + t446 * t28269 / 3.0 + t446 * t28273 / 3.0 + t446 * t28278 / 3.0;
    (t28267, t28269, t28273, t28276, t28278, t28281)
}
