//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 291/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk291<F: Float>(t1249: F, t397: F, t539: F, t535: F, t533: F, t1308: F, sigma0: F) -> (F, F, F) {
    let t1576 = t397 * t1249 * t539;
    let t1578 = F::new(0.89953943580886586067e-2) * t535 * t1576;
    let t1579 = t533 * sigma0;
    let t1580 = t1579 * t1308;
    (t1576, t1578, t1580)
}
