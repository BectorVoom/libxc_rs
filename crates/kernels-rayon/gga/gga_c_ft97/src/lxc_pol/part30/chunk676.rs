//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 676/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk676(t25044: f64, t992: f64, t2665: f64, t446: f64, t28719: f64, t852: f64, t1486: f64, t193: f64, t681: f64, t7083: f64, t89: f64, t24995: f64, t25010: f64, t28770: f64, t28774: f64, t28779: f64, t28783: f64, t28784: f64, t28790: f64, t28794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28796 = t25044 * t992;
    let t28797 = t2665 * t28796;
    let t28798 = t446 * t28797;
    let t28800 = t852 * t28719;
    let t28802 = t1486 * t193 * t28800;
    let t28804 = t681 * t7083;
    let t28805 = t89 * t28804;
    let t28807 = -t28770 / 9.0_f64 + t28774 / 27.0_f64 - t28779 / 6.0_f64 - t28783 - t28784 / 54.0_f64 + t24995 / 9.0_f64 - t25010 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t28790 - 2.0_f64 * t28794 + t28798 / 9.0_f64 - t28802 / 6.0_f64 - 2.0_f64 / 9.0_f64 * t28805;
    (t28796, t28798, t28802, t28804, t28805, t28807)
}
