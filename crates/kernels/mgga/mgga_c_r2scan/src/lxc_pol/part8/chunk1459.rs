//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1459/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1459<F: Float>(t23835: F, t23906: F, t32969: F, t32970: F, t32971: F, t32972: F, t34837: F, t34840: F, t34843: F, t34847: F, t34850: F, t19421: F, t19424: F, t23910: F, t23918: F, t32973: F, t32974: F, t32976: F, t32977: F, t32978: F, t32979: F, t34854: F) -> (F, F) {
    let t35265 = t23835 + t32969 + t34837 + t34840 + t32970 + t34843 - t34847 + t32971 + t32972 + t34850 + t23906;
    let t35267 = t23910 + t32973 + t32974 - t23918 - t32976 - t19421 + t34854 - t32977 - t19424 - t32978 - t32979;
    (t35265, t35267)
}
