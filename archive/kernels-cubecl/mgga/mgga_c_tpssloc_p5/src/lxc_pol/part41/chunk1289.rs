//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1289/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1289<F: Float>(t2281: F, t2331: F, t656: F, t9398: F, t99: F, t2196: F, t2585: F, t8181: F, t8185: F, t111: F, t8199: F, t9576: F) -> (F, F, F, F, F, F, F, F) {
    let t110140 = t2281 * t2331;
    let t110143 = t2281 * t656;
    let t110314 = t99 * t9398;
    let t110333 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t2585 * t2196;
    let t110334 = t110140 * t8181;
    let t110336 = t110143 * t8185;
    let t110363 = t8199 * t111;
    let t110532 = t9576 * t656;
    (t110140, t110143, t110314, t110333, t110334, t110336, t110363, t110532)
}
