//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1037/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1037<F: Float>(t15330: F, t880: F, t2977: F, t861: F, t73: F, t2980: F, t88: F, t15318: F, t3011: F, t98: F, t15283: F, t3015: F) -> (F, F, F, F) {
    let t15331 = t15330 * t880;
    let t15335 = F::new(1.0) / t2977 / t861;
    let t15336 = t73 * t15335;
    let t15338 = F::new(1.0) / t2980 / t88;
    let t15339 = t15318 * t15338;
    let t15343 = F::new(1.0) / t3011 / t98;
    let t15345 = t15343 * t15283 * t3015;
    (t15331, t15336, t15339, t15345)
}
