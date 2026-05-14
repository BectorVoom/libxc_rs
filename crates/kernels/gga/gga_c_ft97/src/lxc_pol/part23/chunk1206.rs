//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1206/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1206<F: Float>(t13927: F, t27986: F, t31262: F, t681: F, t89: F, t30859: F, t713: F, t1882: F, t31226: F, t110702: F, t110713: F, t110718: F, t110719: F, t110733: F, t110735: F, t111016: F, t14127: F, t18196: F, t1901: F, t242: F, t2574: F, t265: F, t31178: F, t3859: F, t42939: F, t446: F, t6154: F, t729: F, t762: F, t766: F, t97790: F) -> (F, F, F) {
    let t122592 = t13927 * t27986;
    let t122598 = t89 * t681 * t31262;
    let t122609 = t30859 * t713;
    let t122622 = t1882 * t31226;
    let t122624 = -t97790 + 4.0 / 3.0 * t446 * t242 * t122592 + 4.0 / 27.0 * t110702 - t122598 / 9.0 + t110713 - t110718 - 8.0 / 27.0 * t110719 + 2.0 / 3.0 * t446 * t729 * t6154 * t18196 - 4.0 / 3.0 * t1901 * t14127 * t111016 * t3859 + 2.0 / 3.0 * t446 * t2574 * t265 * t122609 + t446 * t729 * t762 * t30859 * t766 / 3.0 + 2.0 / 27.0 * t1901 * t42939 * t31178 - 4.0 / 9.0 * t122622 + t110733 - t110735;
    (t122592, t122609, t122624)
}
