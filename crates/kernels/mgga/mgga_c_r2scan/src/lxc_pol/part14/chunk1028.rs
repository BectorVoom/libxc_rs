//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1028/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1028<F: Float>(t3308: F, t37961: F, t7368: F, t10776: F, t7429: F, t10781: F, t7505: F, t11837: F, t1584: F, t26307: F, t574: F, t3309: F, t7566: F, t10725: F, t2651: F, t37754: F, t546: F) -> (F, F, F, F, F, F, F, F) {
    let t40016 = t37961 * t3308 * t7368;
    let t40019 = t10776 * t3308 * t7429;
    let t40021 = t10781 * t7505;
    let t40024 = t1584 * t11837;
    let t40027 = t574 * t3308 * t26307;
    let t40029 = t7566 * t3309;
    let t40031 = t2651 * t10725;
    let t40033 = t546 * t37754;
    (t40016, t40019, t40021, t40024, t40027, t40029, t40031, t40033)
}
