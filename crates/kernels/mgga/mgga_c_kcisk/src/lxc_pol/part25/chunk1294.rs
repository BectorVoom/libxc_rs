//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1294/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1294<F: Float>(t116474: F, t9664: F, t17182: F, t34077: F, t74846: F, t9663: F, t112184: F, t112266: F, t112289: F, t116276: F, t116448: F, t116453: F, t116465: F, t116469: F, t32952: F, t33031: F, t33061: F, t34013: F, t34039: F, t34073: F, t63573: F, t7261: F) -> (F, F, F) {
    let t116476 = 0.69444444444444444446e-2 * t9664 * t116474;
    let t116477 = t17182 * t34077;
    let t116479 = 0.13888888888888888889e-1 * t9664 * t116477;
    let t116482 = t9663 * t74846;
    let t116485 = 0.69444444444444444446e-2 * t112289 * t34013 + 0.34722222222222222223e-2 * t33031 * t116448 - 0.13888888888888888889e-1 * t33031 * t116453 + 0.13888888888888888889e-1 * t112266 * t34039 + 0.13888888888888888889e-1 * t112289 * t34039 + 0.62500000000000000002e-1 * t9664 * t7261 * t112184 * t63573 + 0.77160493827160493827e-3 * t116465 + 0.20833333333333333334e-1 * t9664 * t116469 + 0.62500000000000000002e-1 * t9664 * t116276 - t116476 - t116479 - 0.46296296296296296297e-2 * t34073 * t32952 - 0.18518518518518518519e-1 * t116482 * t33061;
    (t116477, t116482, t116485)
}
