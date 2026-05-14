//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1246/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1246<F: Float>(t35309: F, t35311: F, t35314: F, t35317: F, t35319: F, t35321: F, t35323: F, t35326: F, t35328: F, t35330: F, t35332: F, t35334: F, t35360: F, t10039: F, t2666: F, t8514: F, t9741: F) -> (F, F, F) {
    let t35373 = -0.33333333333333333334e0 * t35309 + 0.9375e-1 * t35311 - 0.20833333333333333333e-1 * t35314 + 0.625e-1 * t35317 - 0.20234375e-1 * t35319 - 0.68347222222222222224e0 * t35321 + 0.28777777777777777778e0 * t35323 - 0.9375e-1 * t35326 - 0.5e0 * t35328 + 0.125e0 * t35330 + 0.20234375e-1 * t35332 - 0.1875e0 * t35334;
    let t35374 = t35360 + t35373;
    let t35378 = t10039 * t2666;
    let t35382 = t9741 * t8514;
    (t35374, t35378, t35382)
}
