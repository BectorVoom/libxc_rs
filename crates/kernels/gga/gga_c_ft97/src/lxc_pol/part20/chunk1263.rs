//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1263/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1263<F: Float>(t14607: F, t25188: F, t29129: F, t310: F, t15073: F, t6353: F, t10683: F, t112654: F, t113334: F, t15157: F, t15308: F, t15313: F, t1901: F, t2409: F, t25220: F, t25225: F, t2801: F, t28924: F, t29198: F, t296: F, t319: F, t4246: F, t446: F, t53797: F, t56854: F, t7021: F, t7131: F, t824: F, t835: F, t840: F, t871: F, t98966: F, t99127: F, t99129: F) -> (F, F, F) {
    let t114089 = t25188 * t14607;
    let t114104 = t310 * t29129;
    let t114120 = t6353 * t15073;
    let t114132 = -2.0 / 9.0 * t99127 + t99129 / 27.0 + 2.0 / 3.0 * t446 * t296 * t112654 + 2.0 / 3.0 * t446 * t840 * t871 * t28924 * t824 + 2.0 / 3.0 * t446 * t296 * t114089 + 2.0 / 3.0 * t446 * t840 * t4246 * t25220 + t446 * t840 * t4246 * t25225 / 3.0 + 4.0 / 9.0 * t53797 * t98966 * t15308 + 4.0 / 3.0 * t53797 * t114104 * t15313 + 2.0 / 9.0 * t446 * t835 * t7131 * t2409 + 2.0 / 3.0 * t446 * t840 * t6353 * t15157 - 2.0 * t446 * t10683 * t319 * t113334 - t446 * t296 * t114120 / 3.0 + t446 * t840 * t871 * t7021 * t2801 / 3.0 - 4.0 / 9.0 * t1901 * t56854 * t29198;
    (t114089, t114120, t114132)
}
