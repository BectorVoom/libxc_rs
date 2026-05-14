//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1298/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1298<F: Float>(t1882: F, t30469: F, t30496: F, t30390: F, t11593: F, t12709: F, t13140: F, t16169: F, t17118: F, t17123: F, t17181: F, t17409: F, t1901: F, t2097: F, t2185: F, t23455: F, t27015: F, t27064: F, t27215: F, t27228: F, t30281: F, t30472: F, t30489: F, t3441: F, t379: F, t446: F, t4668: F, t50773: F, t569: F, t574: F, t5869: F, t5975: F, t63052: F, t6685: F, t9144: F) -> (F,) {
    let t120276 = t1882 * t30469;
    let t120282 = t1882 * t30496;
    let t120284 = t1882 * t30390;
    let t120324 = t446 * t574 * t17409 * t5869 / 3.0 - 2.0 / 9.0 * t120276 - t446 * t569 * t30281 * t379 / 9.0 - t120282 / 9.0 - 2.0 / 9.0 * t120284 - 4.0 / 3.0 * t1901 * t13140 * t23455 * t17118 - 4.0 / 3.0 * t1901 * t13140 * t23455 * t17123 + 2.0 / 3.0 * t446 * t2185 * t5975 * t4668 - 8.0 / 9.0 * t11593 * t12709 * t27215 * t16169 - 2.0 / 9.0 * t1901 * t9144 * t30472 * t379 - 4.0 / 9.0 * t1901 * t63052 * t27064 - 2.0 / 9.0 * t1901 * t50773 * t27228 - 2.0 / 9.0 * t1901 * t9144 * t30489 * t379 - 4.0 / 3.0 * t1901 * t13140 * t27015 * t17181 - 4.0 / 27.0 * t1901 * t2097 * t6685 * t3441;
    (t120324,)
}
