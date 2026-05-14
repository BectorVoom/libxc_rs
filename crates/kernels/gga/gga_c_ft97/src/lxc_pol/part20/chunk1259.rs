//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1259/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1259<F: Float>(t10580: F, t1495: F, t1882: F, t29178: F, t7098: F, t8232: F, t2801: F, t28859: F, t56456: F, t6273: F, t25271: F, t310: F, t10447: F, t112952: F, t11593: F, t1212: F, t14686: F, t14690: F, t15225: F, t15284: F, t15295: F, t15312: F, t15349: F, t15387: F, t15514: F, t1901: F, t2409: F, t25183: F, t29093: F, t29154: F, t296: F, t446: F, t53797: F, t54032: F, t6353: F, t7114: F, t840: F, t871: F, t99672: F) -> (F, F, F) {
    let t113903 = t10580 * t1495;
    let t113914 = 2.0 / 9.0 * t1882 * t29178;
    let t113915 = t8232 * t7098;
    let t113922 = t28859 * t2801;
    let t113932 = t56456 * t6273;
    let t113939 = t310 * t25271;
    let t113947 = 2.0 / 3.0 * t446 * t840 * t6353 * t15284 - 2.0 / 27.0 * t1901 * t29093 * t15349 - 10.0 / 81.0 * t1901 * t113903 * t15387 + 8.0 / 27.0 * t11593 * t29093 * t15514 - 4.0 / 9.0 * t11593 * t10447 * t29154 + t113914 - 4.0 / 27.0 * t113915 + t446 * t840 * t871 * t25183 * t1212 / 3.0 - t446 * t296 * t113922 / 3.0 - 4.0 / 27.0 * t54032 * t112952 * t15295 + 4.0 / 9.0 * t53797 * t99672 * t15225 + 8.0 / 9.0 * t53797 * t113932 * t14690 - 8.0 / 27.0 * t54032 * t113932 * t14686 - 8.0 / 27.0 * t54032 * t113939 * t15295 + 4.0 / 9.0 * t1901 * t15312 * t7114 * t2409;
    (t113922, t113939, t113947)
}
