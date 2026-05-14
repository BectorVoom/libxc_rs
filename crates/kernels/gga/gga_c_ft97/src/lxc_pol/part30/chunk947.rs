//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 947/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk947<F: Float>(t1882: F, t35555: F, t10157: F, t1091: F, t110478: F, t110539: F, t111330: F, t1175: F, t14175: F, t141916: F, t142135: F, t142137: F, t142146: F, t142190: F, t149837: F, t150238: F, t1901: F, t2347: F, t2360: F, t242: F, t2469: F, t2574: F, t2606: F, t265: F, t27753: F, t27757: F, t27763: F, t28023: F, t28246: F, t28378: F, t33452: F, t35516: F, t35624: F, t35697: F, t3886: F, t42575: F, t446: F, t51687: F, t6154: F, t6166: F, t729: F, t7546: F, t773: F, t97777: F) -> (F,) {
    let t152157 = t1882 * t35555;
    let t152159 = 2.0 / 3.0 * t446 * t729 * t28023 * t6166 + 2.0 / 9.0 * t142135 + 4.0 / 3.0 * t446 * t242 * t149837 + 2.0 / 3.0 * t142137 + 2.0 / 9.0 * t142146 - 4.0 / 9.0 * t1901 * t111330 * t27753 - 2.0 / 9.0 * t1901 * t97777 * t28378 - 4.0 / 9.0 * t1901 * t110478 * t27757 + 4.0 / 27.0 * t1901 * t110539 * t27763 + 4.0 / 9.0 * t1901 * t14175 * t7546 * t2360 * t3886 - 4.0 / 27.0 * t1901 * t51687 * t7546 * t2347 * t3886 - 2.0 / 9.0 * t1901 * t42575 * t35624 - t446 * t729 * t773 * t35516 / 3.0 - t446 * t729 * t1175 * t33452 / 3.0 + t1901 * t2606 * t141916 * t1091 / 9.0 - 2.0 / 3.0 * t446 * t2574 * t2469 * t35697 - 2.0 * t446 * t10157 * t265 * t150238 + 2.0 / 3.0 * t446 * t729 * t6154 * t28246 - 2.0 / 9.0 * t142190 - 2.0 / 9.0 * t152157;
    (t152159,)
}
