//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1295/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1295<F: Float>(t2142: F, t30357: F, t1384: F, t61330: F, t26590: F, t3565: F, t1349: F, t30107: F, t376: F, t30458: F, t8392: F, t106395: F, t107627: F, t11593: F, t119558: F, t12709: F, t12714: F, t13212: F, t13220: F, t16011: F, t16150: F, t16950: F, t16955: F, t17006: F, t17011: F, t17380: F, t1901: F, t23548: F, t27207: F, t27211: F, t27215: F, t27220: F, t27256: F, t50773: F, t63304: F, t9144: F) -> (F, F, F, F, F) {
    let t120112 = t2142 * t30357;
    let t120115 = t61330 * t1384;
    let t120117 = t26590 * t3565;
    let t120120 = t1349 * t376 * t30107;
    let t120166 = t8392 * t30458;
    let t120171 = -2.0 / 9.0 * t1901 * t50773 * t27256 + 8.0 / 27.0 * t11593 * t13212 * t119558 + 2.0 / 9.0 * t1901 * t9144 * t27215 * t16950 + 4.0 / 9.0 * t1901 * t13220 * t27215 * t16955 + 2.0 / 3.0 * t1901 * t12709 * t27220 * t16150 - 4.0 / 9.0 * t1901 * t12714 * t106395 * t16150 - 2.0 / 9.0 * t1901 * t50773 * t27207 - 4.0 / 9.0 * t1901 * t63304 * t27211 - t1901 * t9144 * t23548 * t17006 / 9.0 - 2.0 / 9.0 * t1901 * t13220 * t23548 * t17011 - 2.0 / 9.0 * t1901 * t12709 * t27215 * t16011 + 2.0 / 27.0 * t1901 * t12714 * t27220 * t16011 + 2.0 / 27.0 * t120166 + 4.0 / 27.0 * t1901 * t107627 * t17380;
    (t120112, t120115, t120117, t120120, t120171)
}
