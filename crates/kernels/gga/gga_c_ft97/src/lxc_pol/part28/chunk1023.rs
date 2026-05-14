//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1023/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1023<F: Float>(t1882: F, t35052: F, t35198: F, t35166: F, t35095: F, t8392: F, t32992: F, t3578: F, t35103: F, t106698: F, t106803: F, t13208: F, t13212: F, t13220: F, t140137: F, t144: F, t148336: F, t148417: F, t148475: F, t1557: F, t1570: F, t1901: F, t2185: F, t23470: F, t27021: F, t27216: F, t27221: F, t27334: F, t3188: F, t3420: F, t3450: F, t3478: F, t3483: F, t35033: F, t379: F, t39652: F, t446: F, t47659: F, t47666: F, t50240: F, t50744: F, t63755: F, t7357: F, t7400: F, t7414: F) -> (F, F) {
    let t149005 = t1882 * t35052;
    let t149007 = t1882 * t35198;
    let t149034 = t1882 * t35166;
    let t149044 = t8392 * t35095;
    let t149058 = t3578 * t32992;
    let t149062 = t1882 * t35103;
    let t149067 = 2.0 / 3.0 * t446 * t2185 * t7414 * t3450 + 2.0 / 9.0 * t149005 - 2.0 / 9.0 * t149007 - 4.0 / 27.0 * t1901 * t50744 * t148417 - 2.0 / 9.0 * t1901 * t13208 * t148336 + 2.0 / 27.0 * t1901 * t13212 * t148475 + 4.0 / 9.0 * t1901 * t13220 * t7400 * t1570 * t3188 - 4.0 / 27.0 * t1901 * t50240 * t7400 * t1557 * t3188 + 4.0 / 9.0 * t47659 * t106803 * t27216 - 4.0 / 27.0 * t47666 * t106803 * t27221 + t149034 / 9.0 + 8.0 * t1901 * t27334 * t39652 * t7400 * t3483 + 2.0 / 9.0 * t1901 * t23470 * t27021 + 2.0 / 27.0 * t149044 + 8.0 / 3.0 * t1901 * t63755 * t7357 * t3478 + 4.0 * t1901 * t106698 * t7357 * t3483 - 4.0 / 9.0 * t1901 * t13220 * t35033 * t379 - t446 * t144 * t149058 / 3.0 - 4.0 / 9.0 * t149062 + t1901 * t140137 * t3420 / 9.0;
    (t149058, t149067)
}
