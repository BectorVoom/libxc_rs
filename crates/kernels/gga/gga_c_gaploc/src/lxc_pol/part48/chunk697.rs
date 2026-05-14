//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 697/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk697<F: Float>(t34777: F, t901: F, t35106: F, t40261: F, t10418: F, t2389: F, t10215: F, t1429: F, t2365: F, t2366: F, t20535: F, t34688: F, t9537: F, t20671: F, t31037: F, t35101: F) -> (F, F, F, F, F, F, F) {
    let t41960 = t34777 * t901;
    let t41962 = t35106 * t901;
    let t41981 = 0.12780975317973583225e0 * t40261;
    let t42001 = t10418 * t2389;
    let t42026 = t1429 * t2365 * t2366 * t10215;
    let t42066 = t20535 * t34688 * t9537;
    let t42071 = t31037 * t20671 * t35101;
    (t41960, t41962, t41981, t42001, t42026, t42066, t42071)
}
