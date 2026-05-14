//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 685/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk685<F: Float>(t1: F, t31740: F, t544: F, t2875: F, t6514: F, t1559: F, t986: F, t10241: F, t1359: F, t12380: F, t455: F, t145: F, t459: F, t12385: F, t2281: F, t1246: F, t135: F, t4074: F, t458: F, t9105: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35106 = t544 * t31740 * t1;
    let t35180 = t544 * t6514 * t2875;
    let t35204 = t544 * t1559 * t986 * t1;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    let t39632 = t9105 * t4074 * M_PI * t1246 * t135 * t458;
    (t35106, t35180, t35204, t35215, t35216, t39622, t39624, t39626, t39632)
}
