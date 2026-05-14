//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 986/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk986<F: Float>(t31029: F, t31033: F, t31037: F, t31039: F, t31041: F, t31045: F, t31049: F, t31060: F, t31074: F, t31081: F, t31083: F, t35259: F, t35261: F, t35264: F, t35271: F, t35274: F, t35278: F, t35279: F) -> (F,) {
    let t35280 = 0.22921875e-1 * t31029 + 0.4584375e-1 * t31033 + t31037 + 0.80031500487063509016e-2 * t31039 - 0.42874018118069736972e-3 * t31041 + t35259 - t35261 + 0.31448092289604152068e-3 * t35264 - 0.32155513588552302729e-2 * t31045 + 0.14291339372689912324e-3 * t31049 + 0.94344276868812456204e-3 * t31060 - 0.10718504529517434243e-3 * t35271 + t35274 + 0.17149607247227894789e-2 * t31074 - 0.420234375e-1 * t31081 - 0.28015625e-1 * t31083 - t35278 - t35279;
    (t35280,)
}
