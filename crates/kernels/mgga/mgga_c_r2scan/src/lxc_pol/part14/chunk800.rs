//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 800/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk800<F: Float>(t4703: F, t4880: F, t4891: F, t6943: F, t6946: F, t6947: F, t6948: F, t6949: F, t6950: F, t6951: F, t6952: F, t2461: F, t759: F, t761: F, t4721: F, t4901: F, t4964: F, t4967: F, t4972: F, t4975: F, t4979: F, t4981: F, t6954: F, t6960: F) -> (F, F) {
    let t7858 = -t6943 - t4880 + t6946 - t6947 - t6948 + t4891 + t6949 + t6950 - t4703 + t6951 + t6952;
    let t7861 = 0.571528e-1 * t759 * t2461 * t761;
    let t7862 = -t4901 + t7861 - t4721 + t4964 - t4967 - t6954 - t4972 + t4975 - t6960 + t4979 + t4981;
    (t7858, t7862)
}
