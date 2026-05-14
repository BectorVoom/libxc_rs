//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1275/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1275<F: Float>(t19421: F, t19424: F, t23916: F, t23918: F, t23920: F, t23922: F, t23926: F, t23927: F, t23928: F, t23929: F, t23932: F, t23935: F, t19476: F, t19478: F, t2813: F, t6887: F) -> (F, F, F, F) {
    let t23936 = -t23916 - t23918 + t23920 + t19421 + t23922 + t23926 + t19424 + t23927 + t23928 - t23929 - t23932 - t23935;
    let t23937 = 360.0 * t19476;
    let t23938 = 72.0 * t19478;
    let t23939 = t6887 * t2813;
    (t23936, t23937, t23938, t23939)
}
