//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 718/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk718<F: Float>(t11167: F, t655: F, t10585: F, t7234: F, t1785: F, t4648: F, t5015: F, t5014: F, t5030: F, t1636: F, t5032: F, t4644: F, sigma2: F) -> (F, F, F, F, F) {
    let t11168 = t11167 * sigma2;
    let t11169 = t11168 * t655;
    let t11172 = t7234 * t10585;
    let t11175 = t4648 * t1785;
    let t11176 = t5015 * t11175;
    let t11179 = t5014 * t5030;
    let t11180 = t1636 * t5032;
    let t11181 = t11179 * t11180;
    let t11184 = t4644 * t1785;
    (t11169, t11172, t11176, t11181, t11184)
}
