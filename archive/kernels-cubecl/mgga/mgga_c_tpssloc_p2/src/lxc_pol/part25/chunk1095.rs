//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1095/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1095<F: Float>(t1369: F, t80866: F, t22782: F, t3777: F, t22783: F, t3876: F, t22788: F, t12361: F, t6952: F, t15: F, t2229: F, t1361: F, t192: F, t1995: F, t22690: F) -> (F, F, F, F, F, F, F) {
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    let t80870 = t80869 * t1369;
    let t80872 = t22783 * t3876;
    let t80876 = t22788 * t3876;
    let t80878 = t6952 * t12361;
    let t80881 = F::cast_from(1.0_f64) / t2229 / t15;
    let t80885 = t80881 * t1995 * t192 * t22690 * t1361;
    (t80867, t80870, t80872, t80876, t80878, t80881, t80885)
}
