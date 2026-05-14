//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1277/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1277<F: Float>(t10261: F, t871: F, t28924: F, t870: F, t1882: F, t29396: F, t29290: F, t29295: F, t10703: F, t113041: F, t113316: F, t113341: F, t11593: F, t15229: F, t15299: F, t15485: F, t1901: F, t25044: F, t2862: F, t2867: F, t2881: F, t29222: F, t4260: F, t44369: F, t446: F, t6353: F, t684: F, t7036: F, t99635: F, t99644: F, t99646: F, t99656: F) -> (F,) {
    let t114531 = t10261 * t871;
    let t114554 = t870 * t28924;
    let t114565 = 2.0 / 9.0 * t1882 * t29396;
    let t114567 = 4.0 / 9.0 * t1882 * t29290;
    let t114569 = 2.0 / 9.0 * t1882 * t29295;
    let t114570 = 4.0 * t1901 * t114531 * t7036 * t2867 + 2.0 / 3.0 * t1901 * t15229 * t113341 - 2.0 / 9.0 * t1901 * t15299 * t113041 + 8.0 / 9.0 * t11593 * t15299 * t113316 - 2.0 / 9.0 * t1901 * t44369 * t29222 - 2.0 / 9.0 * t1901 * t10703 * t25044 * t4260 + t99635 + 2.0 / 9.0 * t99644 - 2.0 / 27.0 * t99646 + 2.0 / 9.0 * t1901 * t2881 * t114554 * t684 - 2.0 / 9.0 * t99656 - 2.0 / 3.0 * t446 * t2862 * t6353 * t15485 - t114565 - t114567 - t114569;
    (t114570,)
}
