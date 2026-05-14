//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1264/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1264<F: Float>(t1882: F, t29346: F, t10688: F, t28854: F, t29366: F, t29151: F, t8392: F, t10666: F, t7124: F, t112407: F, t1212: F, t15162: F, t15369: F, t1901: F, t24898: F, t25382: F, t25474: F, t2682: F, t2862: F, t29141: F, t29193: F, t296: F, t4246: F, t4311: F, t44302: F, t44369: F, t446: F, t56815: F, t6278: F, t7101: F, t840: F, t871: F, t99140: F) -> (F, F, F) {
    let t114142 = 2.0 / 9.0 * t1882 * t29346;
    let t114150 = t10688 * t28854;
    let t114162 = 4.0 / 9.0 * t1882 * t29366;
    let t114164 = 2.0 / 27.0 * t8392 * t29151;
    let t114177 = t10666 * t7124;
    let t114181 = -2.0 / 9.0 * t1901 * t44369 * t29193 - 2.0 / 3.0 * t446 * t2862 * t4246 * t25382 - t114142 + 4.0 / 3.0 * t446 * t2862 * t4311 * t6278 - 4.0 / 3.0 * t1901 * t56815 * t29141 + 4.0 / 3.0 * t446 * t296 * t114150 - 2.0 / 3.0 * t1901 * t15369 * t24898 * t15162 - 2.0 * t446 * t296 * t112407 - t114162 - t114164 - t99140 + t1901 * t44302 * t7101 / 9.0 - 2.0 / 3.0 * t446 * t2862 * t871 * t7124 * t2682 - t446 * t840 * t25474 * t1212 / 3.0 - t446 * t296 * t114177 / 3.0;
    (t114150, t114177, t114181)
}
