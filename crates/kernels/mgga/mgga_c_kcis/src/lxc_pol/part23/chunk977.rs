//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 977/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk977<F: Float>(t187: F, t27490: F, t27493: F, t27496: F, t27497: F, t27498: F, t27500: F, t27502: F, t27505: F, t27508: F, t27511: F, t27554: F, t27676: F, t27713: F, t449: F, t446: F) -> (F, F, F) {
    let t27716 = t27490 - t27493 + t27496 - t27497 - t27498 + t27500 - t27502 - t27505 + t27508 + t27511 - t27554 + t187 * (t27676 + t27713);
    let t27717 = t449 * t27716;
    let t27718 = t446 * t27717;
    (t27716, t27717, t27718)
}
