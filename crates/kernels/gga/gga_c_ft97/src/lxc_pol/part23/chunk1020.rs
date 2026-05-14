//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1020/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1020<F: Float>(t14163: F, t30986: F, t5170: F, t6135: F, t10007: F, t242: F, t31064: F, t1091: F, t6947: F, t724: F, t2574: F, t265: F, t31029: F, t1901: F, t24658: F, t28106: F, t28113: F, t28191: F, t28212: F, t28214: F, t31102: F, t31107: F, t31111: F, t31115: F, t31119: F, t31123: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t31126 = t14163 * t30986;
    let t31129 = t6135 * t5170;
    let t31130 = t10007 * t31129;
    let t31135 = t242 * t31064;
    let t31139 = t724 * t6947 * t1091;
    let t31143 = t2574 * t265 * t31029;
    let t31146 = 2.0 / 9.0 * t28106 - 2.0 / 9.0 * t28113 - t446 * t31102 / 3.0 + 2.0 / 9.0 * t28191 + t1901 * t31107 / 9.0 + 2.0 / 27.0 * t1901 * t31111 - 2.0 / 9.0 * t1901 * t31115 - 4.0 / 3.0 * t1901 * t31119 - t446 * t31123 / 3.0 - 4.0 / 9.0 * t1901 * t31126 - 2.0 / 9.0 * t1901 * t31130 + 2.0 / 9.0 * t28212 + 2.0 / 9.0 * t28214 - 2.0 * t446 * t31135 - 2.0 / 9.0 * t446 * t31139 + 2.0 / 3.0 * t446 * t31143 + t24658;
    (t31126, t31129, t31130, t31135, t31139, t31143, t31146)
}
