//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 948/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk948<F: Float>(t87252: F, t87285: F, t1017: F, t20022: F, t1053: F, t1060: F, t12714: F, t13212: F, t1901: F, t20023: F, t20045: F, t20660: F, t20709: F, t20743: F, t20763: F, t2179: F, t2185: F, t2205: F, t2983: F, t3578: F, t4454: F, t446: F, t4462: F, t4668: F, t4714: F, t4724: F, t4733: F, t4805: F, t4839: F, t50773: F, t569: F, t574: F, t605: F, t86977: F, t9144: F, t9327: F, t9432: F) -> (F, F, F, F) {
    let t87286 = t87252 + t87285;
    let t87295 = t20022 * t1017;
    let t87303 = t20022 * t1053;
    let t87372 = -8.0 * t446 * t2185 * t3578 * t20709 - 4.0 * t446 * t2185 * t605 * t4668 * t4805 + 8.0 * t446 * t9432 * t605 * t20660 * t1053 - 4.0 / 3.0 * t1901 * t9144 * t4462 * t4733 + 8.0 / 9.0 * t1901 * t12714 * t2983 * t20763 + 8.0 / 9.0 * t1901 * t13212 * t86977 - 8.0 / 3.0 * t1901 * t50773 * t20743 - 4.0 / 9.0 * t446 * t569 * t1060 * t20045 - 4.0 / 9.0 * t446 * t2205 * t4839 * t4454 - 4.0 * t446 * t574 * t2179 * t4714 * t4724 - 40.0 / 81.0 * t446 * t9327 * t1060 * t20023;
    (t87286, t87295, t87303, t87372)
}
