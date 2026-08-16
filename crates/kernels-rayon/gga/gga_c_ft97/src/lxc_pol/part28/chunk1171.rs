//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1171/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1171(t1882: f64, t35052: f64, t35198: f64, t35166: f64, t35095: f64, t8392: f64, t32992: f64, t3578: f64, t35103: f64, t106698: f64, t106803: f64, t13208: f64, t13212: f64, t13220: f64, t140137: f64, t144: f64, t148336: f64, t148417: f64, t148475: f64, t1557: f64, t1570: f64, t1901: f64, t2185: f64, t23470: f64, t27021: f64, t27216: f64, t27221: f64, t27334: f64, t3188: f64, t3420: f64, t3450: f64, t3478: f64, t3483: f64, t35033: f64, t379: f64, t39652: f64, t446: f64, t47659: f64, t47666: f64, t50240: f64, t50744: f64, t63755: f64, t7357: f64, t7400: f64, t7414: f64) -> (f64, f64) {
    let t149005 = t1882 * t35052;
    let t149007 = t1882 * t35198;
    let t149034 = t1882 * t35166;
    let t149044 = t8392 * t35095;
    let t149058 = t3578 * t32992;
    let t149062 = t1882 * t35103;
    let t149067 = 2.0_f64 / 3.0_f64 * t446 * t2185 * t7414 * t3450 + 2.0_f64 / 9.0_f64 * t149005 - 2.0_f64 / 9.0_f64 * t149007 - 4.0_f64 / 27.0_f64 * t1901 * t50744 * t148417 - 2.0_f64 / 9.0_f64 * t1901 * t13208 * t148336 + 2.0_f64 / 27.0_f64 * t1901 * t13212 * t148475 + 4.0_f64 / 9.0_f64 * t1901 * t13220 * t7400 * t1570 * t3188 - 4.0_f64 / 27.0_f64 * t1901 * t50240 * t7400 * t1557 * t3188 + 4.0_f64 / 9.0_f64 * t47659 * t106803 * t27216 - 4.0_f64 / 27.0_f64 * t47666 * t106803 * t27221 + t149034 / 9.0_f64 + 8.0_f64 * t1901 * t27334 * t39652 * t7400 * t3483 + 2.0_f64 / 9.0_f64 * t1901 * t23470 * t27021 + 2.0_f64 / 27.0_f64 * t149044 + 8.0_f64 / 3.0_f64 * t1901 * t63755 * t7357 * t3478 + 4.0_f64 * t1901 * t106698 * t7357 * t3483 - 4.0_f64 / 9.0_f64 * t1901 * t13220 * t35033 * t379 - t446 * t144 * t149058 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t149062 + t1901 * t140137 * t3420 / 9.0_f64;
    (t149058, t149067)
}
