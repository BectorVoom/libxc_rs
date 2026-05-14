//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1031/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1031<F: Float>(t6371: F, t8232: F, t6388: F, t6367: F, t6376: F, t1882: F, t25291: F, t10478: F, t1495: F, t38953: F, t6275: F, t24891: F, t8392: F, t6355: F, t25277: F, t6260: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t98751 = t8232 * t6371;
    let t98753 = t8232 * t6388;
    let t98788 = t8232 * t6367;
    let t98790 = t8232 * t6376;
    let t98800 = t1882 * t25291;
    let t98809 = t10478 * t1495;
    let t98823 = t38953 * t6275;
    let t98840 = t8392 * t24891;
    let t98850 = t8232 * t6355;
    let t98880 = t8392 * t25277;
    let t98899 = t870 * t6260;
    (t98751, t98753, t98788, t98790, t98800, t98809, t98823, t98840, t98850, t98880, t98899)
}
