//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1273/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1273<F: Float>(t112: F, t8199: F, t111: F, t2205: F, t2585: F, t656: F, t1849: F, t8189: F, t2199: F, t5361: F, t1266: F, t8273: F) -> (F, F, F, F, F, F) {
    let t30109 = t8199 * t112;
    let t30112 = t2205 * t111;
    let t30175 = t2585 * t656;
    let t30266 = t8189 * t1849;
    let t30269 = t2199 * t5361;
    let t30272 = t1266 * t8273;
    (t30109, t30112, t30175, t30266, t30269, t30272)
}
