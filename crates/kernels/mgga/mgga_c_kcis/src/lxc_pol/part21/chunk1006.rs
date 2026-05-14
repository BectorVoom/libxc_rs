//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1006/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1006<F: Float>(t26991: F, t27037: F, t27058: F, t27093: F, t1281: F, t7807: F, t1291: F, t7823: F, t26889: F, t26892: F, t26894: F, t26898: F, t26900: F, t26902: F, t26904: F, t26906: F, t26908: F, t26910: F, t26912: F, t26914: F) -> (F, F, F, F) {
    let t27095 = t26991 + t27037 + t27058 + t27093;
    let t27100 = t7807 * t1281;
    let t27105 = t7823 * t1291;
    let t27120 = -0.4046875e-1 * t26889 + 0.5e0 * t26892 - 0.125e0 * t26894 + 0.1875e0 * t26898 - 0.26979166666666666667e-1 * t26900 + 0.20234375e-1 * t26902 + 0.21583333333333333334e0 * t26904 - 0.53958333333333333334e-1 * t26906 + 0.4046875e-1 * t26908 + 0.28777777777777777778e0 * t26910 - 0.68347222222222222224e0 * t26912 - 0.89930555555555555557e-2 * t26914;
    (t27095, t27100, t27105, t27120)
}
