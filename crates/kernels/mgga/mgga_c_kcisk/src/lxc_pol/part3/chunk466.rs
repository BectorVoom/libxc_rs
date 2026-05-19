//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 466/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk466<F: Float>(t1379: F, t311: F, t313: F, t1187: F, t827: F, t1311: F, t79: F, t3575: F, t26: F, t1186: F, t3579: F, t3583: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3657 = t311 * t1379 * t313;
    let t3658 = F::cast_from(0.13692777777777777778e0_f64) * t3657;
    let t3659 = t827 * t1187;
    let t3661 = t79 * t1311;
    let t3662 = t3661 * t3575;
    let t3663 = t26 * t3662;
    let t3665 = t1186 * t3579;
    let t3666 = t26 * t3665;
    let t3668 = t1186 * t3583;
    (t3657, t3658, t3659, t3661, t3662, t3663, t3665, t3666, t3668)
}
