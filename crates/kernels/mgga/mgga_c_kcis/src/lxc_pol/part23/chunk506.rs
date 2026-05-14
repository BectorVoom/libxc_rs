//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 506/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk506<F: Float>(t4254: F, t4124: F, t556: F, t572: F, t1532: F, t492: F, sigma2: F) -> (F, F, F, F, F) {
    let t4255 = t4254 * sigma2;
    let t4256 = t556 * t4124;
    let t4257 = t572 * t4256;
    let t4258 = t4255 * t4257;
    let t4260 = t1532 * t492;
    (t4255, t4256, t4257, t4258, t4260)
}
