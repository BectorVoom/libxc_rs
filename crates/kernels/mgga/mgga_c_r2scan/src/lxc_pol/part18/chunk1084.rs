//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1084/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1084<F: Float>(t3290: F, t9302: F, t12486: F, t24039: F, t10856: F, t9236: F, t10698: F, t12506: F, t12517: F, t1584: F, t29936: F, t3308: F, t574: F, t12520: F, t12463: F, t2207: F, t3336: F) -> (F, F, F, F, F, F, F, F) {
    let t43688 = t3290 * t9302;
    let t43690 = t24039 * t12486;
    let t43692 = t10856 * t9236;
    let t43695 = t10698 * t12506;
    let t43697 = t1584 * t12517;
    let t43700 = t574 * t3308 * t29936;
    let t43702 = t1584 * t12520;
    let t43705 = t2207 * t3336 * t12463;
    (t43688, t43690, t43692, t43695, t43697, t43700, t43702, t43705)
}
