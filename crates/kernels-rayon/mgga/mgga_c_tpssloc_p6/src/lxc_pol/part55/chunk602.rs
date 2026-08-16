//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 602/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk602(t1118: f64, t4781: f64, t1099: f64, t1670: f64, t3315: f64, t1117: f64, t3313: f64, t3238: f64, t3319: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64) -> (f64, f64, f64) {
    let t4782 = t4781 * t1118;
    let t4784 = 1.0_f64 * t1099 * t4782;
    let t4785 = t1670 * t3315;
    let t4786 = t4785 * t1117;
    let t4788 = 0.16081979498692535067e2_f64 * t3313 * t4786;
    let t4794 = t3319 - 0.57077777777777777777e-2_f64 * t3238 - 0.57077777777777777777e-2_f64 * t4721 - 0.11415555555555555555e-1_f64 * t4726 + 0.34246666666666666666e-1_f64 * t4731 + 0.17123333333333333333e-1_f64 * t4735;
    (t4784, t4788, t4794)
}
