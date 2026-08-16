//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1127/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1127<F: Float>(t381: F, t7577: F, t23384: F, t7554: F, t7607: F, t225: F) -> (F, F, F, F) {
    let t25442 = t7577 * t381;
    let t25450 = t23384 * t7554;
    let t25465 = t23384 * t7607;
    let t25470 = t7577 * t225;
    (t25442, t25450, t25465, t25470)
}
