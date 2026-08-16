//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1150/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1150<F: Float>(t1058: F, t1060: F, t2207: F, t7088: F, t3308: F, t37961: F, t7368: F, t10776: F, t7429: F, t10781: F, t7505: F, t11837: F, t1584: F) -> (F, F, F, F, F) {
    let t40011 = t2207 * t1058 * t1060 * t7088;
    let t40016 = t37961 * t3308 * t7368;
    let t40019 = t10776 * t3308 * t7429;
    let t40021 = t10781 * t7505;
    let t40024 = t1584 * t11837;
    (t40011, t40016, t40019, t40021, t40024)
}
