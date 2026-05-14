//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1253/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1253<F: Float>(t7091: F, t848: F, t10703: F, t112376: F, t112585: F, t113007: F, t113009: F, t113017: F, t113640: F, t15538: F, t1901: F, t2409: F, t24886: F, t2739: F, t2766: F, t28719: F, t2884: F, t296: F, t319: F, t4141: F, t4256: F, t446: F, t6347: F, t7045: F, t7124: F, t840: F, t871: F, t882: F, t98840: F, t98850: F, t99186: F) -> (F,) {
    let t113656 = t848 * t7091;
    let t113660 = 8.0 / 27.0 * t113007 + 22.0 / 27.0 * t113009 + t446 * t840 * t871 * t7124 * t2739 / 3.0 + t113017 - 4.0 / 27.0 * t1901 * t2766 * t6347 * t4141 + 2.0 / 9.0 * t1901 * t99186 * t4256 - 2.0 / 27.0 * t98840 - 8.0 / 27.0 * t98850 + 2.0 / 3.0 * t446 * t296 * t112585 + t1901 * t24886 * t15538 / 9.0 - t446 * t296 * t113640 / 3.0 - 2.0 / 3.0 * t446 * t840 * t882 * t28719 - t446 * t840 * t319 * t112376 / 3.0 + 2.0 / 9.0 * t1901 * t10703 * t7045 * t2409 + 2.0 / 9.0 * t1901 * t113656 * t2884;
    (t113660,)
}
