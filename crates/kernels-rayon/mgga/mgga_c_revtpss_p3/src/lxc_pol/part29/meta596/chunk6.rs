//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2009/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2009(t26519: f64, t98867: f64, t103212: f64, t103216: f64, t103219: f64, t103220: f64, t103224: f64, t103234: f64, t1579: f64, t26473: f64, t2828: f64, t2829: f64, t28394: f64, t7070: f64, t7071: f64, t7997: f64, t887: f64, t95807: f64, t95808: f64, t95811: f64, t95813: f64, t95823: f64) -> f64 {
    let t103240 = t98867 * t26519;
    let t103242 = t95807 - 0.45699670022203476294e-2_f64 * t95808 - 0.13170898365871023197e1_f64 * t103212 * t887 + t103216 + 0.48186823267806663678e-3_f64 * t95811 - t103219 + 0.13009920719177044025e-1_f64 * t103220 - 0.45699670022203476294e-2_f64 * t95813 - t103224 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t7997 * t2828 - 0.65854491829355115987e0_f64 * t28394 * t2829 + 0.28912093960683998208e-1_f64 * t95823 - 0.24093411633903331839e-3_f64 * t103234 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t26473 * t1579 - 0.22849835011101738147e-2_f64 * t103240;
    t103242
}
