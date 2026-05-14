//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 675/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk675<F: Float>(t14364: F, t380: F, t470: F, t140: F, t446: F, t480: F, t12951: F, t451: F, t12825: F, t41: F, t12829: F, t13329: F, t492: F, t1555: F, t524: F, t4349: F, t544: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14365 = t14364 * sigma0;
    let t14374 = 1.0 / t470 / t380;
    let t14409 = 0.11791604938271604938e-1 * t140 * t446 * t480;
    let t14484 = t451 * t12951;
    let t14496 = t41 * t12825;
    let t14497 = t451 * t12829;
    let t14545 = t13329 * t492;
    let t14607 = t1555 * t1555;
    let t14608 = 1.0 / t14607;
    let t14609 = t524 * t14608;
    let t14612 = 1.0 / t4349 / t544;
    (t14365, t14374, t14409, t14484, t14496, t14497, t14545, t14609, t14612)
}
