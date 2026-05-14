//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1027/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1027<F: Float>(t1882: F, t36191: F, t34053: F, t4246: F, t36179: F, t25188: F, t28930: F, t112760: F, t112975: F, t1212: F, t144093: F, t144094: F, t144096: F, t144105: F, t144107: F, t15133: F, t152673: F, t152678: F, t152722: F, t15299: F, t1901: F, t29189: F, t296: F, t33961: F, t33994: F, t34118: F, t34225: F, t4176: F, t4181: F, t4260: F, t44528: F, t446: F, t56643: F, t72190: F, t7629: F, t840: F) -> (F, F, F) {
    let t154538 = t1882 * t36191;
    let t154550 = t4246 * t34053;
    let t154568 = t1882 * t36179;
    let t154586 = t25188 * t28930;
    let t154590 = 2.0 / 3.0 * t446 * t840 * t4246 * t34118 - 2.0 / 9.0 * t154538 - t144093 - 2.0 / 9.0 * t144094 - 2.0 / 9.0 * t144096 + 8.0 / 3.0 * t1901 * t72190 * t7629 * t4176 + 4.0 * t1901 * t112975 * t7629 * t4181 - t446 * t296 * t154550 / 3.0 + t144105 / 9.0 + t144107 / 9.0 - t446 * t840 * t33994 * t1212 / 3.0 + t446 * t840 * t4246 * t34225 / 3.0 + 2.0 / 3.0 * t446 * t840 * t15133 * t7629 + 2.0 / 9.0 * t154568 + 2.0 / 9.0 * t1901 * t44528 * t33961 * t4260 - 4.0 / 9.0 * t1901 * t15299 * t152722 + 4.0 / 9.0 * t1901 * t15299 * t152673 - 4.0 / 27.0 * t1901 * t56643 * t152678 - 4.0 / 9.0 * t1901 * t112760 * t29189 + 4.0 / 3.0 * t446 * t296 * t154586;
    (t154550, t154586, t154590)
}
