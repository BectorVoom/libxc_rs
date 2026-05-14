//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1366/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1366<F: Float>(t105372: F, t105448: F, t106182: F, t12680: F, t12737: F, t12742: F, t13212: F, t13220: F, t144: F, t1643: F, t1647: F, t1651: F, t1901: F, t2097: F, t2190: F, t2210: F, t23464: F, t23470: F, t26849: F, t27020: F, t3441: F, t379: F, t446: F, t50240: F, t50744: F, t5916: F, t5929: F, t63755: F, t6699: F, t6708: F, t9144: F, t95573: F, t95601: F) -> (F,) {
    let t106550 = -4.0 / 27.0 * t1901 * t2097 * t5929 * t3441 + 2.0 / 9.0 * t1901 * t23470 * t12742 + 2.0 / 9.0 * t1901 * t12680 * t23464 + 4.0 / 27.0 * t1901 * t13212 * t105448 + 2.0 / 3.0 * t446 * t144 * t106182 - t1901 * t9144 * t5916 * t12737 / 9.0 - 4.0 / 27.0 * t1901 * t50744 * t105372 + 2.0 / 27.0 * t95573 + 8.0 / 3.0 * t1901 * t63755 * t6699 * t2190 - 2.0 / 9.0 * t1901 * t13220 * t6708 * t1651 - 4.0 / 27.0 * t1901 * t50240 * t6708 * t1643 + 2.0 / 9.0 * t1901 * t9144 * t6699 * t1647 - 2.0 / 9.0 * t1901 * t9144 * t26849 * t379 + 4.0 / 9.0 * t95601 + t1901 * t2210 * t27020 * t1651 / 9.0;
    (t106550,)
}
