//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1393/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1393<F: Float>(t15133: F, t7124: F, t1882: F, t31828: F, t10443: F, t1091: F, t113710: F, t113939: F, t11593: F, t15191: F, t15254: F, t15312: F, t18514: F, t1901: F, t19409: F, t19465: F, t19622: F, t19626: F, t2749: F, t2874: F, t29067: F, t29071: F, t29202: F, t29207: F, t296: F, t31735: F, t31743: F, t31852: F, t4311: F, t44178: F, t446: F, t53797: F, t54032: F, t6273: F, t7021: F, t840: F, t99229: F, t99271: F) -> (F, F) {
    let t128004 = t15133 * t7124;
    let t128023 = t1882 * t31828;
    let t128032 = -4.0 / 9.0 * t11593 * t15191 * t29067 + 2.0 / 9.0 * t1901 * t10443 * t31735 + 2.0 / 9.0 * t1901 * t2874 * t113710 * t1091 + 4.0 / 9.0 * t1901 * t15312 * t29202 * t19465 + 2.0 / 3.0 * t1901 * t15254 * t29207 * t18514 + t99229 - 2.0 / 3.0 * t446 * t296 * t128004 + t446 * t840 * t2749 * t31852 / 3.0 - 4.0 * t1901 * t29071 * t6273 * t19409 - 2.0 / 3.0 * t446 * t840 * t4311 * t7021 + 2.0 / 27.0 * t1901 * t44178 * t31743 + 2.0 / 9.0 * t128023 - 4.0 / 27.0 * t99271 + 8.0 / 9.0 * t53797 * t113939 * t19622 - 8.0 / 27.0 * t54032 * t113939 * t19626;
    (t128004, t128032)
}
