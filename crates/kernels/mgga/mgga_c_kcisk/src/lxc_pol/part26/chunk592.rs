//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 592/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk592<F: Float>(t5835: F, t5863: F, t1216: F, t1349: F, t1402: F, t2110: F, t2209: F, t338: F, t3814: F, t3815: F, t3817: F, t3819: F, t3857: F, t417: F, t451: F, t5641: F, t5643: F, t5647: F, t5650: F, t5653: F, t5655: F, t5659: F, t5704: F, t5798: F) -> (F, F) {
    let t5864 = t5835 + t5863;
    let t5866 = t3814 + 0.23426533963880895498e-2 * t3815 + 0.46853067927761790996e-2 * t3817 + 0.23426533963880895498e-2 * t5641 + 0.46853067927761790996e-2 * t3819 * t5643 + 0.46853067927761790996e-2 * t1349 * t5647 - 0.46853067927761790996e-2 * t3857 * t5650 + 0.46853067927761790996e-2 * t5653 + 0.46853067927761790996e-2 * t1349 * t5655 + 0.14055920378328537299e-1 * t417 * t5659 - 0.46853067927761790996e-2 * t417 * t5704 - t5798 * t451 - t2110 * t1402 - t1216 * t2209 - t338 * t5864;
    (t5864, t5866)
}
