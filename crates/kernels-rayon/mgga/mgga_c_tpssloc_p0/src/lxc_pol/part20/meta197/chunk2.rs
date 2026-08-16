//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1191/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1191(t1117: f64, t4785: f64, t3313: f64, t3238: f64, t3319: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t1128: f64, t1675: f64, t1136: f64, t1683: f64) -> (f64, f64, f64, f64, f64) {
    let t4786 = t4785 * t1117;
    let t4788 = 0.16081979498692535067e2_f64 * t3313 * t4786;
    let t4794 = t3319 - 0.57077777777777777777e-2_f64 * t3238 - 0.57077777777777777777e-2_f64 * t4721 - 0.11415555555555555555e-1_f64 * t4726 + 0.34246666666666666666e-1_f64 * t4731 + 0.17123333333333333333e-1_f64 * t4735;
    let t4797 = t1675 * t1128;
    let t4802 = t1683 * t1136;
    (t4786, t4788, t4794, t4797, t4802)
}
