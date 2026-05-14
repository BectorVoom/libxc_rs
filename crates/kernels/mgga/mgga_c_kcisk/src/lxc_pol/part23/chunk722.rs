//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 722/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk722<F: Float>(t4230: F, t6373: F, t470: F, t5967: F, t487: F, t1487: F, t2275: F, t4223: F, t492: F, t5885: F, t1506: F, t1483: F, t2267: F, t1413: F, t2257: F, t1489: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6374 = t4230 * t6373;
    let t6376 = t470 * t5967;
    let t6377 = t487 * t6376;
    let t6378 = t1487 * t6377;
    let t6380 = t4223 * t2275;
    let t6382 = t5885 * t492;
    let t6383 = t6382 * t1506;
    let t6385 = t1483 * t2267;
    let t6387 = t2257 * t1413;
    let t6388 = t6387 * sigma0;
    let t6389 = t6388 * t1489;
    (t6374, t6376, t6377, t6378, t6380, t6382, t6383, t6385, t6387, t6388, t6389)
}
