//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1307/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1307<F: Float>(t106551: F, t106588: F, t106623: F, t106724: F, t11593: F, t119503: F, t12680: F, t13153: F, t1391: F, t144: F, t17031: F, t17044: F, t17048: F, t17066: F, t17099: F, t17181: F, t17203: F, t17207: F, t1901: F, t2185: F, t2221: F, t26863: F, t26918: F, t26924: F, t26999: F, t27007: F, t27334: F, t27335: F, t30423: F, t3420: F, t446: F, t5942: F, t64242: F, t925: F, t9419: F) -> (F,) {
    let t120748 = 2.0 / 9.0 * t1901 * t9419 * t30423 + 2.0 / 9.0 * t1901 * t2221 * t106724 * t925 + 2.0 / 9.0 * t1901 * t13153 * t27007 + 2.0 / 9.0 * t1901 * t106551 * t3420 - 4.0 * t1901 * t27334 * t27335 * t17181 + 2.0 * t1901 * t26999 * t5942 * t17066 + 2.0 / 27.0 * t1901 * t26863 * t17044 + 4.0 / 9.0 * t1901 * t26863 * t17048 - 10.0 / 81.0 * t1901 * t106588 * t17203 - 8.0 / 27.0 * t11593 * t26863 * t17207 - 4.0 / 3.0 * t1901 * t64242 * t26924 + 4.0 / 9.0 * t11593 * t12680 * t26918 - 4.0 / 3.0 * t1901 * t106623 * t17031 + 2.0 / 3.0 * t446 * t2185 * t1391 * t17099 + 4.0 / 3.0 * t446 * t144 * t119503;
    (t120748,)
}
