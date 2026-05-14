//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 866/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk866<F: Float>(t1454: F, t6837: F, t1403: F, t35266: F, t681: F, t35269: F, t107910: F, t1091: F, t109713: F, t140513: F, t141422: F, t193: F, t2354: F, t24204: F, t24231: F, t27993: F, t28001: F, t28026: F, t28033: F, t28039: F, t33499: F, t35251: F, t6002: F, t6008: F, t684: F) -> (F,) {
    let t149715 = t6837 * t1454;
    let t149725 = t1403 * t681 * t35266;
    let t149728 = t1403 * t681 * t35269;
    let t149738 = -t6002 * t140513 * t28026 / 3.0 + 2.0 / 9.0 * t6002 * t24231 * t28001 - t33499 * t27993 / 18.0 - t24204 * t35251 / 18.0 - t6002 * t2354 * t141422 * t1091 / 18.0 - t6002 * t2354 * t149715 * t684 / 9.0 + t33499 * t28033 / 9.0 - t33499 * t28039 / 27.0 - t149725 / 3.0 + 2.0 / 9.0 * t149728 - 2.0 / 3.0 * t1403 * t193 * t6008 * t109713 - 2.0 / 3.0 * t1403 * t193 * t6008 * t107910;
    (t149738,)
}
