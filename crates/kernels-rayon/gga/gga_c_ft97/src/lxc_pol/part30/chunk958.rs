//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 958/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk958(t1882: f64, t33668: f64, t33587: f64, t5996: f64, t1506: f64, t6260: f64, t1476: f64, t6391: f64, t7611: f64, t880: f64, t34312: f64, t6213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142423 = t1882 * t33668;
    let t142434 = t5996 * t33587;
    let t142455 = t6260 * t1506;
    let t142460 = t1476 * t6391;
    let t142485 = t7611 * t880;
    let t142501 = t34312 * t6213;
    (t142423, t142434, t142455, t142460, t142485, t142501)
}
