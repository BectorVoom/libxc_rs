//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1285/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1285<F: Float>(t309: F, t6273: F, t7640: F, t29219: F, t8392: F, t10703: F, t111703: F, t113320: F, t11593: F, t14678: F, t1508: F, t15229: F, t15303: F, t15425: F, t15441: F, t15522: F, t1901: F, t28520: F, t2862: F, t296: F, t3281: F, t446: F, t53797: F, t56448: F, t6334: F, t668: F, t835: F, t99672: F, t99895: F, t99909: F, t99911: F, t99916: F, t99923: F) -> (F,) {
    let t114907 = t7640 * t309 * t6273;
    let t114935 = 4.0 / 27.0 * t8392 * t29219;
    let t114936 = -2.0 / 3.0 * t446 * t296 * t111703 - t99895 / 9.0 + 2.0 / 3.0 * t446 * t2862 * t1508 * t15425 + t99909 / 9.0 - 2.0 / 9.0 * t99911 + 4.0 / 3.0 * t53797 * t114907 * t14678 + 4.0 / 9.0 * t53797 * t99672 * t15303 + 2.0 / 9.0 * t3281 * t835 * t1508 * t668 + t99916 / 9.0 + 2.0 / 9.0 * t99923 - t1901 * t10703 * t6334 * t15522 / 9.0 + 4.0 / 9.0 * t11593 * t10703 * t6334 * t15441 - 4.0 / 9.0 * t1901 * t56448 * t28520 - 4.0 / 9.0 * t1901 * t15229 * t113320 + t114935;
    (t114936,)
}
