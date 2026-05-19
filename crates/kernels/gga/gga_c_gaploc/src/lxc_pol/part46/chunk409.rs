//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 409/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk409<F: Float>(t2859: F, t3377: F, t2366: F, t986: F, t2365: F, t1429: F, t1457: F, t3354: F, t1572: F, t3358: F, t531: F, t2778: F, t874: F) -> (F, F, F, F, F, F, F, F) {
    let t3379 = F::cast_from(0.10725146985555128001e1_f64) * t2859 * t3377;
    let t3380 = t2366 * t986;
    let t3381 = t2365 * t3380;
    let t3382 = t1429 * t3381;
    let t3383 = F::cast_from(0.14896037479937677779e-1_f64) * t3382;
    let t3384 = t1457 * t3354;
    let t3386 = F::cast_from(0.71500979903700853338e0_f64) * t1572 * t3384;
    let t3387 = t531 * t3358;
    let t3390 = t2778 * t874;
    (t3379, t3380, t3381, t3383, t3384, t3386, t3387, t3390)
}
