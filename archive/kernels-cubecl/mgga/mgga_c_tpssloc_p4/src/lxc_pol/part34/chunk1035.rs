//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1035/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1035<F: Float>(t193: F, t2061: F, t532: F, t7939: F, t12571: F, t7025: F, t23967: F, t7432: F, t7032: F, t7435: F, t7428: F, t2031: F, t26012: F) -> (F, F, F, F, F, F, F) {
    let t26756 = t193 * t2061;
    let t26905 = t532 * t7939;
    let t26911 = t12571 * t7025;
    let t26920 = t23967 * t7432;
    let t26936 = t7435 * t7032;
    let t26948 = t7428 * t7032;
    let t26954 = t2031 * t26012;
    (t26756, t26905, t26911, t26920, t26936, t26948, t26954)
}
