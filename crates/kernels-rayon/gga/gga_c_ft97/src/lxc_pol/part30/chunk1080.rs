//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1080/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1080(t1882: f64, t35555: f64, t10157: f64, t1091: f64, t110478: f64, t110539: f64, t111330: f64, t1175: f64, t14175: f64, t141916: f64, t142135: f64, t142137: f64, t142146: f64, t142190: f64, t149837: f64, t150238: f64, t1901: f64, t2347: f64, t2360: f64, t242: f64, t2469: f64, t2574: f64, t2606: f64, t265: f64, t27753: f64, t27757: f64, t27763: f64, t28023: f64, t28246: f64, t28378: f64, t33452: f64, t35516: f64, t35624: f64, t35697: f64, t3886: f64, t42575: f64, t446: f64, t51687: f64, t6154: f64, t6166: f64, t729: f64, t7546: f64, t773: f64, t97777: f64) -> f64 {
    let t152157 = t1882 * t35555;
    let t152159 = 2.0_f64 / 3.0_f64 * t446 * t729 * t28023 * t6166 + 2.0_f64 / 9.0_f64 * t142135 + 4.0_f64 / 3.0_f64 * t446 * t242 * t149837 + 2.0_f64 / 3.0_f64 * t142137 + 2.0_f64 / 9.0_f64 * t142146 - 4.0_f64 / 9.0_f64 * t1901 * t111330 * t27753 - 2.0_f64 / 9.0_f64 * t1901 * t97777 * t28378 - 4.0_f64 / 9.0_f64 * t1901 * t110478 * t27757 + 4.0_f64 / 27.0_f64 * t1901 * t110539 * t27763 + 4.0_f64 / 9.0_f64 * t1901 * t14175 * t7546 * t2360 * t3886 - 4.0_f64 / 27.0_f64 * t1901 * t51687 * t7546 * t2347 * t3886 - 2.0_f64 / 9.0_f64 * t1901 * t42575 * t35624 - t446 * t729 * t773 * t35516 / 3.0_f64 - t446 * t729 * t1175 * t33452 / 3.0_f64 + t1901 * t2606 * t141916 * t1091 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t2574 * t2469 * t35697 - 2.0_f64 * t446 * t10157 * t265 * t150238 + 2.0_f64 / 3.0_f64 * t446 * t729 * t6154 * t28246 - 2.0_f64 / 9.0_f64 * t142190 - 2.0_f64 / 9.0_f64 * t152157;
    t152159
}
