//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2009/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2009<F: Float>(t26519: F, t98867: F, t103212: F, t103216: F, t103219: F, t103220: F, t103224: F, t103234: F, t1579: F, t26473: F, t2828: F, t2829: F, t28394: F, t7070: F, t7071: F, t7997: F, t887: F, t95807: F, t95808: F, t95811: F, t95813: F, t95823: F) -> F {
    let t103240 = t98867 * t26519;
    let t103242 = t95807 - F::cast_from(0.45699670022203476294e-2_f64) * t95808 - F::cast_from(0.13170898365871023197e1_f64) * t103212 * t887 + t103216 + F::cast_from(0.48186823267806663678e-3_f64) * t95811 - t103219 + F::cast_from(0.13009920719177044025e-1_f64) * t103220 - F::cast_from(0.45699670022203476294e-2_f64) * t95813 - t103224 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t7997 * t2828 - F::cast_from(0.65854491829355115987e0_f64) * t28394 * t2829 + F::cast_from(0.28912093960683998208e-1_f64) * t95823 - F::cast_from(0.24093411633903331839e-3_f64) * t103234 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t26473 * t1579 - F::cast_from(0.22849835011101738147e-2_f64) * t103240;
    t103242
}
