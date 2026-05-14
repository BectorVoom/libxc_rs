//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1161/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1161<F: Float>(t1785: F, t2469: F, t32935: F, t7261: F, t17078: F, t648: F, t1772: F) -> (F, F, F, F) {
    let t34146 = t2469 * t1785;
    let t34147 = t32935 * t34146;
    let t34148 = t7261 * t34147;
    let t34153 = t17078 * t648;
    let t34154 = t34153 * t1772;
    (t34147, t34148, t34153, t34154)
}
