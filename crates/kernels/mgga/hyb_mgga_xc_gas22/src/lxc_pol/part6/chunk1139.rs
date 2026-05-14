//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1139/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1139<F: Float>(t1890: F, t7966: F, t3011: F, t6012: F, t19754: F, t39: F, t7962: F, t7948: F, t7942: F, t7958: F, t7975: F, t7971: F, t3017: F, t33: F, t6022: F, t6025: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23257 = t1890 * t7966;
    let t23284 = t6012 * t3011;
    let t23295 = t19754 * t39;
    let t23311 = t1890 * t7962;
    let t23313 = t1890 * t7948;
    let t23315 = t7942 * t7958;
    let t23317 = t1890 * t7975;
    let t23319 = t7942 * t7971;
    let t23321 = t6012 * t3017;
    let t23323 = t33 * param_hyb_omega_0;
    let t23328 = t6022 * t39 * t6025;
    (t23257, t23284, t23295, t23311, t23313, t23315, t23317, t23319, t23321, t23323, t23328)
}
