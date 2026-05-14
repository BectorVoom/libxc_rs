//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1208/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1208<F: Float>(t1457: F, t1572: F, t31711: F, t10463: F, t4950: F, t10477: F, t17551: F, t3384: F, t204: F, t2476: F, t32033: F, t123: F, t7861: F, t883: F, t2487: F, t2488: F) -> (F, F, F, F, F, F, F) {
    let t34352 = 0.14300195980740170668e1 * t1572 * t1457 * t31711;
    let t34354 = 0.14300195980740170668e1 * t4950 * t10463;
    let t34356 = 0.14300195980740170668e1 * t4950 * t10477;
    let t34358 = 0.71500979903700853338e0 * t17551 * t3384;
    let t34361 = 0.18404604457881959845e2 * t2476 * t204 * t32033;
    let t34363 = t7861 * t123 * t883;
    let t34365 = t2487 * t2488 * t34363;
    (t34352, t34354, t34356, t34358, t34361, t34363, t34365)
}
