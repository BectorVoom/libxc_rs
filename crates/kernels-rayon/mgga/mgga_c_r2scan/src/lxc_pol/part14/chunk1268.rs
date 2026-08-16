//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1268/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1268(t10610: f64, t3472: f64, t40487: f64, t1115: f64, t2526: f64, t3270: f64, t10667: f64, t3262: f64, t40523: f64, t38303: f64, t38308: f64, t39116: f64, t39117: f64, t39121: f64, t40659: f64, t40672: f64, t42302: f64, t42304: f64, t42307: f64, t42310: f64, t42313: f64) -> (f64, f64, f64, f64) {
    let t42316 = 15.0_f64 / 8.0_f64 * t10610 * t3472 * t40487;
    let t42318 = t3270 * t1115 * t2526;
    let t42320 = 3.0_f64 / 2.0_f64 * t10667 * t42318;
    let t42326 = 15.0_f64 / 16.0_f64 * t3262 * t3472 * t40523;
    let t42327 = t42302 - t42304 - t39116 + 0.68400385060046895e-6_f64 * t40659 + t42307 + t42310 + t42313 + t42316 - t42320 - 0.7044137609176975208e-2_f64 * t40672 - t39117 - 0.2881692658299671676e-2_f64 * t38303 + 0.72042316457491791901e-3_f64 * t38308 + t39121 + t42326;
    (t42316, t42320, t42326, t42327)
}
