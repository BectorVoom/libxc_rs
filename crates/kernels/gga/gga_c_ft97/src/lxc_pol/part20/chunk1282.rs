//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1282/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1282<F: Float>(t29131: F, t8392: F, t29212: F, t56110: F, t6360: F, t113056: F, t113126: F, t113152: F, t113197: F, t113524: F, t113939: F, t11593: F, t14686: F, t14690: F, t15225: F, t15255: F, t15290: F, t15299: F, t1901: F, t2405: F, t2749: F, t2862: F, t29369: F, t319: F, t44518: F, t446: F, t53797: F, t54032: F, t56339: F, t7105: F, t840: F, t98966: F, t99717: F, t99719: F) -> (F,) {
    let t114770 = 4.0 / 3.0 * t8392 * t29131;
    let t114772 = 4.0 / 27.0 * t8392 * t29212;
    let t114792 = t56110 * t6360;
    let t114806 = 2.0 / 9.0 * t99717 + 2.0 / 9.0 * t99719 + 2.0 / 3.0 * t446 * t840 * t2749 * t29369 + t114770 + t114772 - 4.0 / 9.0 * t1901 * t15299 * t113126 - 4.0 / 9.0 * t1901 * t15299 * t113197 + 10.0 / 81.0 * t1901 * t56339 * t113056 - 8.0 / 27.0 * t11593 * t15290 * t113152 - 2.0 / 27.0 * t1901 * t44518 * t7105 * t2405 + 4.0 / 9.0 * t53797 * t98966 * t15225 + 4.0 / 9.0 * t53797 * t114792 * t14690 - 4.0 / 27.0 * t54032 * t114792 * t14686 + 8.0 / 9.0 * t53797 * t113939 * t15255 + 2.0 / 3.0 * t446 * t2862 * t319 * t113524;
    (t114806,)
}
