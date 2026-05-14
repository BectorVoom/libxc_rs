//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1296/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1296<F: Float>(t30451: F, t8392: F, t106214: F, t106729: F, t119571: F, t119596: F, t119608: F, t119612: F, t119657: F, t119735: F, t119750: F, t119760: F, t12703: F, t12709: F, t13208: F, t13212: F, t1570: F, t17011: F, t1901: F, t30494: F, t3188: F, t3478: F, t3483: F, t379: F, t49579: F, t5916: F, t63755: F, t6630: F, t6718: F, t9144: F) -> (F,) {
    let t120206 = t8392 * t30451;
    let t120219 = -4.0 / 9.0 * t1901 * t13208 * t119735 + 4.0 / 27.0 * t1901 * t13212 * t119657 - t1901 * t9144 * t30494 * t379 / 9.0 + t106214 - 4.0 / 9.0 * t1901 * t13212 * t119750 + 10.0 / 81.0 * t1901 * t49579 * t119760 - 2.0 / 9.0 * t1901 * t12703 * t119608 - t1901 * t9144 * t5916 * t17011 / 9.0 - 2.0 / 9.0 * t1901 * t13208 * t119612 + 4.0 * t1901 * t106729 * t6630 * t3478 + 8.0 / 3.0 * t1901 * t63755 * t6630 * t3483 + 4.0 / 27.0 * t120206 - 4.0 / 9.0 * t1901 * t12703 * t119571 - 2.0 / 9.0 * t1901 * t12703 * t119596 - 4.0 / 9.0 * t1901 * t12709 * t6718 * t1570 * t3188;
    (t120219,)
}
