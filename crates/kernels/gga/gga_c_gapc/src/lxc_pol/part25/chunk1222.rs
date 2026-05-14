//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1222/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1222<F: Float>(t3909: F, t4905: F, t12625: F, t36326: F, t36331: F, t36455: F, t36457: F, t36460: F, t36462: F, t36467: F, t36470: F, t36472: F, t36474: F, t36483: F, t36892: F, t36894: F, t37308: F, t38708: F, t38710: F, t7056: F) -> (F, F) {
    let t38834 = t4905 * t3909;
    let t38835 = 4.0 * t12625 * t7056 - t36326 + t36331 + t36455 + t36457 - t36460 - t36462 + t36467 - t36470 - t36472 - t36474 + t36483 - t36892 - t36894 - t37308 + t38708 - t38710 + t38834;
    (t38834, t38835)
}
