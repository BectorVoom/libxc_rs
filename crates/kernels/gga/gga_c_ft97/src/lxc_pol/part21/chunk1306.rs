//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1306/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1306<F: Float>(t1882: F, t30436: F, t30515: F, t1045: F, t106639: F, t11593: F, t119403: F, t119567: F, t119689: F, t119778: F, t12703: F, t144: F, t167: F, t16996: F, t17001: F, t1901: F, t2185: F, t2210: F, t23463: F, t23900: F, t27333: F, t27336: F, t30105: F, t30457: F, t40945: F, t446: F, t4462: F, t4827: F, t574: F, t5856: F, t5916: F, t605: F, t609: F, t63586: F, t9144: F) -> (F,) {
    let t120648 = t1882 * t30436;
    let t120654 = t1882 * t30515;
    let t120696 = -2.0 / 9.0 * t120648 + t1901 * t2210 * t23463 * t4462 / 9.0 + 2.0 / 3.0 * t120654 + 2.0 / 3.0 * t446 * t2185 * t167 * t119689 + t446 * t574 * t605 * t30105 * t609 / 3.0 - 4.0 * t1901 * t27333 * t1045 * t27336 - 4.0 / 9.0 * t1901 * t12703 * t119778 - 8.0 / 9.0 * t11593 * t12703 * t119567 - 2.0 / 9.0 * t1901 * t40945 * t30457 - 2.0 / 9.0 * t1901 * t9144 * t23900 * t4827 - 2.0 / 9.0 * t1901 * t9144 * t5916 * t16996 - 4.0 / 9.0 * t11593 * t9144 * t5916 * t17001 - t446 * t144 * t119403 / 3.0 + t106639 + t1901 * t63586 * t5856 / 9.0;
    (t120696,)
}
