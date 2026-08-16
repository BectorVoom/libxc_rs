//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1928/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1928<F: Float>(t26426: F, t81046: F, t22690: F, t7732: F, t81195: F, t16413: F, t1985: F, t1998: F, t214: F, t16248: F, t22833: F, t16383: F) -> (F, F, F, F, F) {
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    let t91091 = t1985 * t214 * t1998 * t16413;
    let t91094 = t22833 * t16248;
    let t91096 = t22833 * t16383;
    (t91078, t91081, t91091, t91094, t91096)
}
