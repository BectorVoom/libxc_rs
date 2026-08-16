//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 665/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk665(t213: f64, t820: f64, t231: f64, t6819: f64, t1208: f64, t811: f64, t19072: f64, t14721: f64, t14729: f64, t14742: f64, t14766: f64, t25049: f64, t25055: f64, t25112: f64, t25118: f64, t28630: f64, t28634: f64, t28639: f64, t28646: f64, t28652: f64, t28655: f64, t28660: f64, t6045: f64) -> (f64, f64, f64, f64, f64) {
    let t28661 = t213 * t820;
    let t28662 = t231 * t28661;
    let t28663 = t6819 * t28662;
    let t28666 = t1208 * t811;
    let t28667 = t231 * t28666;
    let t28671 = t231 * t19072;
    let t28675 = -0.45306850413028723348e0_f64 * t14721 * t28630 + 0.45306850413028723348e0_f64 * t14742 * t28634 - 0.45306850413028723348e0_f64 * t14721 * t28639 + 0.45306850413028723348e0_f64 * t14766 * t28630 - 0.45306850413028723348e0_f64 * t14729 * t28634 + 0.4445200072839506173e-1_f64 * t28646 + 0.45306850413028723348e0_f64 * t14766 * t28639 - 0.33339000546296296298e-1_f64 * t25055 + 0.33339000546296296298e-1_f64 * t25118 - 0.24167761770734866966e0_f64 * t28652 * t28655 + 0.24167761770734866966e0_f64 * t28660 * t28663 + 0.20003400327777777778e0_f64 * t25049 * t6045 * t28667 - 0.30005100491666666667e0_f64 * t25112 * t6045 * t28671;
    (t28663, t28666, t28667, t28671, t28675)
}
