//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk749;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk750;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta131(t1143: f64, t300: f64, t457: f64, t697: f64, t461: f64, t221: f64, t456: f64, t1176: f64, t135: f64, t1179: f64, t1174: f64, t1186: f64, t1089: f64, t405: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3411, t3426, t3428, t3430, t3431) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk749(t1143, t300, t457, t697, t461, t221, t456, t1176, t135);
        let (t3432, t3433, t3435, t3436, t3439) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk750(t1179, t3431, t1174, t1186, t135, t1089, t405);
        let t3440 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk751(t3439, t974);
    (t3411, t3426, t3428, t3430, t3431, t3432, t3433, t3435, t3436, t3439, t3440)
}
