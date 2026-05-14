//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1256/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1256<F: Float>(t1763: F, t1772: F, t4830: F, t32942: F, t32995: F, t32909: F, t32948: F, t17182: F, t32892: F, t9649: F, t18325: F, t32941: F) -> (F, F, F, F, F, F) {
    let t112244 = t4830 * t1763 * t1772;
    let t112247 = t32942 * t32995;
    let t112249 = t32948 * t32909;
    let t112255 = t17182 * t32892;
    let t112256 = t9649 * t112255;
    let t112266 = t32941 * t18325;
    (t112244, t112247, t112249, t112255, t112256, t112266)
}
