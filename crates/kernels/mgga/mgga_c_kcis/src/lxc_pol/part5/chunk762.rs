//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 762/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk762<F: Float>(t1035: F, t6352: F, t2918: F, t4612: F, t6328: F, t6332: F, t6336: F, t261: F, t1680: F, t4685: F, t1679: F, t950: F, t2938: F, t2960: F, t6320: F, t6338: F, t939: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6353 = t1035 * t6352;
    let t6360 = t2918 + 0.11872222222222222222e-1 * t4612 - 0.11872222222222222222e-1 * t6328 + 0.35616666666666666666e-1 * t6332 - 0.17808333333333333333e-1 * t6336;
    let t6362 = 0.62182e-1 * t6360 * t261;
    let t6364 = 2.0 * t4685 * t1680;
    let t6365 = t1679 * t1679;
    let t6366 = t6365 * t950;
    let t6368 = 2.0 * t2938 * t6366;
    let t6375 = t2960 * t6320;
    let t6377 = t939 * t6338;
    (t6353, t6360, t6362, t6364, t6365, t6366, t6368, t6375, t6377)
}
