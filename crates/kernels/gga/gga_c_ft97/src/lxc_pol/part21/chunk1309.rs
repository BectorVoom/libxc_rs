//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1309/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1309<F: Float>(t106565: F, t106803: F, t106807: F, t107311: F, t107420: F, t11593: F, t119175: F, t120115: F, t12968: F, t13140: F, t13220: F, t144: F, t16671: F, t16675: F, t167: F, t16988: F, t17369: F, t17376: F, t17380: F, t1901: F, t2185: F, t23548: F, t27020: F, t30450: F, t3450: F, t3455: F, t446: F, t4668: F, t47659: F, t47666: F, t4827: F, t4839: F, t51170: F, t574: F, t5860: F, t5968: F, t605: F, t9144: F, t95541: F, t95676: F) -> (F,) {
    let t120839 = -t446 * t574 * t167 * t119175 / 3.0 - 4.0 / 9.0 * t11593 * t9144 * t23548 * t16988 - 4.0 / 9.0 * t1901 * t51170 * t30450 - 4.0 / 9.0 * t1901 * t13220 * t95541 * t4827 + 2.0 / 3.0 * t446 * t2185 * t4839 * t5860 - 4.0 / 3.0 * t1901 * t12968 * t27020 * t3450 - 4.0 / 3.0 * t1901 * t13140 * t107420 * t3455 - 4.0 / 27.0 * t47666 * t107311 * t16675 + 4.0 / 9.0 * t47659 * t106565 * t17369 + 4.0 / 9.0 * t47659 * t106803 * t17376 - 4.0 / 27.0 * t47666 * t106803 * t17380 + 8.0 / 9.0 * t47659 * t106807 * t16671 + 4.0 / 81.0 * t95676 - t446 * t144 * t120115 / 3.0 - 2.0 / 3.0 * t446 * t2185 * t605 * t5968 * t4668;
    (t120839,)
}
