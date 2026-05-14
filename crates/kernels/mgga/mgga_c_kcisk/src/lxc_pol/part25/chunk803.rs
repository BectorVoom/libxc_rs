//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 803/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk803<F: Float>(t10879: F, t1777: F, t1773: F, t4998: F, t5025: F, t25: F, t5005: F) -> (F, F, F) {
    let t10880 = t10879 * t1777;
    let t10881 = t1773 * t10880;
    let t10883 = t4998 * t5025;
    let t10884 = t1773 * t10883;
    let t10886 = t25 * t5005;
    (t10881, t10884, t10886)
}
