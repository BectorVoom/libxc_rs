//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1172/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1172<F: Float>(t28720: F, t375: F, t89: F, t10631: F, t1900: F, t6: F, t91: F, t2770: F, t6318: F, t25026: F, t28738: F, t458: F, t10478: F, t25162: F, t28764: F, t28769: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113446 = t89 * t375 * t28720;
    let t113447 = 2.0 / 3.0 * t113446;
    let t113458 = t91 * t10631 * t6 * t1900;
    let t113459 = t2770 * t6318;
    let t113465 = t25026 * t458 * t28738;
    let t113466 = t113465 / 4.0;
    let t113513 = t10478 * t6318;
    let t113564 = t25162 * t28764;
    let t113565 = 2.0 / 9.0 * t113564;
    let t113566 = t25162 * t28769;
    (t113446, t113447, t113458, t113459, t113465, t113466, t113513, t113564, t113565, t113566)
}
