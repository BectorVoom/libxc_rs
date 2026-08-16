//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 541/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk541<F: Float>(t41: F, t4971: F, t2009: F, t2630: F, t2629: F, t1772: F, sigma2: F) -> (F, F, F, F) {
    let t7568 = t41 * t4971;
    let t7578 = t2630 * t2009;
    let t7580 = t2629 * sigma2;
    let t7581 = t7580 * t1772;
    (t7568, t7578, t7580, t7581)
}
