//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1003/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1003<F: Float>(t24906: F, t37943: F, t37945: F, t24916: F, t37949: F, t2184: F, t25746: F, t3308: F, t10781: F, t7996: F, t10810: F, t574: F, t8066: F, t10697: F, t11669: F, t11671: F) -> (F, F, F, F, F, F) {
    let t39482 = t37943 * t37945 * t24906;
    let t39485 = t37949 * t37945 * t24916;
    let t39490 = t2184 * t3308 * t25746;
    let t39495 = t10781 * t7996;
    let t39499 = t574 * t10810 * t8066;
    let t39502 = t10697 * t11669 * t11671;
    (t39482, t39485, t39490, t39495, t39499, t39502)
}
