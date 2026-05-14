//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1328/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1328<F: Float>(t19421: F, t19424: F, t19611: F, t19614: F, t19720: F, t23904: F, t23906: F, t23910: F, t23916: F, t23918: F, t23920: F, t23922: F, t23927: F, t23928: F, t23929: F, t23937: F, t23938: F) -> (F,) {
    let t25028 = t23904 - t23906 + t23910 + t23916 + t23918 - t23920 - t19421 - t23922 - t19424 - t23927 - t23928 + t23929 - t23937 - t23938 + t19720 + t19611 + t19614;
    (t25028,)
}
