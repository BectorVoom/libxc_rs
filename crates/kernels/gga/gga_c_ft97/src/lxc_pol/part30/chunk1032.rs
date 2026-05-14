//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1032/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1032<F: Float>(t28924: F, t6353: F, t1882: F, t36268: F, t15133: F, t7679: F, t36153: F, t8392: F, t36149: F, t143673: F, t144250: F, t144260: F, t144271: F, t144273: F, t15128: F, t15312: F, t153555: F, t1901: F, t24873: F, t29215: F, t296: F, t34167: F, t4255: F, t4260: F, t44523: F, t446: F, t56352: F, t6386: F, t840: F, t992: F, t99238: F) -> (F, F, F, F, F) {
    let t154842 = t6353 * t28924;
    let t154849 = t1882 * t36268;
    let t154851 = t15133 * t7679;
    let t154864 = t8392 * t36153;
    let t154867 = t8392 * t36149;
    let t154896 = 4.0 / 9.0 * t154867 - 2.0 / 9.0 * t1901 * t99238 * t29215 + 2.0 / 9.0 * t1901 * t44523 * t143673 * t4255 + 2.0 / 3.0 * t1901 * t56352 * t143673 * t4260 - 4.0 / 9.0 * t1901 * t15312 * t24873 * t992 * t6386 - t144250 / 9.0 + 2.0 / 9.0 * t144260 - 2.0 / 3.0 * t446 * t840 * t15128 * t34167 + 2.0 / 3.0 * t446 * t296 * t153555 + 2.0 / 9.0 * t144271 + 2.0 / 3.0 * t144273;
    (t154842, t154849, t154851, t154864, t154896)
}
