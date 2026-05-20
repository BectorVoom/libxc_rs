//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2936/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2936<F: Float>(t11268: F, t4820: F, t247: F, t42792: F, t4757: F, t4837: F, t15850: F, t3111: F, t3091: F, t43240: F, t4782: F, t2251: F, t4186: F) -> (F, F, F, F, F) {
    let t53427 = t11268 * t4820;
    let t53431 = t4837 * t247 * t42792 * t4757;
    let t53432 = F::cast_from(0.28582678745379824648e-3_f64) * t53431;
    let t53433 = t15850 * t3111;
    let t53437 = t3091 * t43240 * t4782;
    let t53450 = t4186 * t2251;
    (t53427, t53432, t53433, t53437, t53450)
}
