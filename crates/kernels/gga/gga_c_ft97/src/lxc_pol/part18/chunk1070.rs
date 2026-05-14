//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1070/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1070<F: Float>(t15564: F, t15565: F, t172: F, t15576: F, t15569: F, t1526: F, t3009: F, t7705: F, t10994: F, t10998: F, t11003: F, t11013: F, t11050: F, t11059: F, t11280: F, t15567: F, t15568: F, t15575: F, t2258: F, t2984: F, t2993: F, t38310: F, t38313: F, t38316: F, t38319: F, t38341: F, t38355: F, t38369: F, t432: F, t61123: F, t8633: F) -> (F, F) {
    let t61128 = t15564 * t15565 * t172;
    let t61130 = t61128 * t15576 / 9.0;
    let t61132 = 2.0 / 27.0 * t61128 * t15569;
    let t61147 = t1526 * t7705 * t3009 / 18.0;
    let t61159 = -t15567 * t15575 * t11050 / 2.0 + 2.0 / 3.0 * t15567 * t15568 * t11059 - 4.0 / 9.0 * t61123 * t15568 * t11013 + t61130 - t61132 - 2.0 / 9.0 * t15567 * t8633 * t432 * t2984 + t38310 / 27.0 - t38313 / 18.0 - t38316 / 36.0 - t1526 * t11280 * t10994 / 3.0 - t38341 / 12.0 - t38319 / 27.0 - t61147 - t38355 + t38369 / 9.0 + t15567 * t15575 * t10998 / 6.0 + 2.0 / 3.0 * t61123 * t15575 * t11003 + t15567 * t2258 * t432 * t2993 / 3.0;
    (t61128, t61159)
}
