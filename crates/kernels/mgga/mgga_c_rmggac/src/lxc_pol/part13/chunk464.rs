//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 464/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk464<F: Float>(t352: F, t5098: F, t4616: F, t570: F, t876: F, t1357: F, t866: F, t1652: F, t874: F, t1615: F, t333: F, t1614: F, t338: F) -> (F, F, F, F, F, F) {
    let t5099 = t5098 * t352;
    let t5102 = t4616 * t570;
    let t5103 = t5102 * t876;
    let t5108 = t1357 * t866;
    let t5115 = t874 * t1652;
    let t5116 = t5115 * t352;
    let t5121 = t1615 * t333;
    let t5126 = t338 * t1614;
    (t5099, t5103, t5108, t5116, t5121, t5126)
}
