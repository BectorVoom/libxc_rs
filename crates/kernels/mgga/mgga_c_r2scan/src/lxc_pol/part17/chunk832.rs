//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 832/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk832<F: Float>(t4721: F, t4964: F, t4967: F, t4972: F, t4975: F, t4979: F, t4981: F, t6961: F, t8555: F, t8556: F, t8559: F, t8560: F, t3128: F, t879: F, t4791: F, t4794: F, t4798: F, t4806: F, t6963: F, t6966: F, t8592: F, t8596: F, t8600: F, t8603: F, t8632: F) -> (F, F) {
    let t9794 = t8555 - t4721 + t4964 - t4967 - t8556 - t4972 + t4975 - t8559 - t8560 + t4979 - t4981 - t6961;
    let t9797 = t879 * t3128;
    let t9798 = t6963 + 2.0 * t6966 + t9797 - t8592 - t4791 + t4794 + t4798 - t4806 - t8596 - t8600 + t8603 - t8632;
    (t9794, t9798)
}
