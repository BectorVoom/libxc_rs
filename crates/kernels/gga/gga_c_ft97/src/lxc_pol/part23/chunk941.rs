//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 941/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk941<F: Float>(t213: F, t820: F, t231: F, t6819: F, t1208: F, t811: F, t19072: F, t14721: F, t14729: F, t14742: F, t14766: F, t25049: F, t25055: F, t25112: F, t25118: F, t28630: F, t28634: F, t28639: F, t28646: F, t28652: F, t28655: F, t28660: F, t6045: F) -> (F, F, F, F, F, F, F) {
    let t28661 = t213 * t820;
    let t28662 = t231 * t28661;
    let t28663 = t6819 * t28662;
    let t28666 = t1208 * t811;
    let t28667 = t231 * t28666;
    let t28671 = t231 * t19072;
    let t28675 = -0.45306850413028723348e0 * t14721 * t28630 + 0.45306850413028723348e0 * t14742 * t28634 - 0.45306850413028723348e0 * t14721 * t28639 + 0.45306850413028723348e0 * t14766 * t28630 - 0.45306850413028723348e0 * t14729 * t28634 + 0.4445200072839506173e-1 * t28646 + 0.45306850413028723348e0 * t14766 * t28639 - 0.33339000546296296298e-1 * t25055 + 0.33339000546296296298e-1 * t25118 - 0.24167761770734866966e0 * t28652 * t28655 + 0.24167761770734866966e0 * t28660 * t28663 + 0.20003400327777777778e0 * t25049 * t6045 * t28667 - 0.30005100491666666667e0 * t25112 * t6045 * t28671;
    (t28661, t28662, t28663, t28666, t28667, t28671, t28675)
}
