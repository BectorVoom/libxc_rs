//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 188/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk188<F: Float>(t4: F, t68: F, t85: F, t73: F, t2: F, t41: F, t74: F, t818: F, t71: F, t163: F, t80: F, t81: F, t88: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t857 = t68 * t4;
    let t861 = t85 * t85;
    let t862 = 1.0 / t861;
    let t863 = t73 * t862;
    let t866 = 1.0 / t74 * t41 * t2;
    let t867 = t866 * t818;
    let t869 = t68 * t818;
    let t871 = f64::sqrt(t71);
    let t873 = t871 * t41 * t2;
    let t874 = t873 * t818;
    let t877 = t80 * t81 * t163;
    let t879 = -0.632975e0 * t867 - 0.29896666666666666667e0 * t869 - 0.1023875e0 * t874 - 0.82156666666666666667e-1 * t877;
    let t880 = 1.0 / t88;
    (t857, t861, t862, t863, t866, t867, t869, t873, t874, t877, t879, t880)
}
