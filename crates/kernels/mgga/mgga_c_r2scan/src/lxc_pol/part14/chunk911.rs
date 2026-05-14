//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 911/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk911<F: Float>(t1058: F, t11824: F, t2207: F, t3333: F, t7601: F, t1584: F, t3597: F, t3308: F, t8066: F, t574: F, t2651: F, t3309: F, t10810: F, t2608: F, t10698: F, t3588: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11826 = t2207 * t1058 * t11824;
    let t11831 = t7601 * t3333;
    let t11835 = t1584 * t3597;
    let t11837 = t3308 * t8066;
    let t11838 = t574 * t11837;
    let t11840 = t2651 * t3309;
    let t11842 = t10810 * t2608;
    let t11843 = t574 * t11842;
    let t11845 = t10698 * t3588;
    (t11826, t11831, t11835, t11837, t11838, t11840, t11842, t11843, t11845)
}
