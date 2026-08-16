//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2768/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2768<F: Float>(t13999: F, t22271: F, t48919: F, t6869: F, t9816: F, t9818: F, t13847: F, t22016: F, t48731: F, t73731: F, t1399: F, t73856: F) -> (F, F, F, F) {
    let t74186 = t13999 * t22271;
    let t74206 = t9816 * t9818 * t48919 * t6869;
    let t74232 = t48731 * t13847 * t73731 * t22016;
    let t74249 = t9816 * t13847 * t73856 * t1399;
    (t74186, t74206, t74232, t74249)
}
