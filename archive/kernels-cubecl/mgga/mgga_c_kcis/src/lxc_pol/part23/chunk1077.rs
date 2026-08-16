//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1077/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1077<F: Float>(t27582: F, t27620: F, t27645: F, t27669: F, t1636: F, t8010: F, t27490: F, t27493: F, t27496: F, t27497: F, t27500: F, t27505: F, t27508: F, t27511: F, t27554: F, t4480: F, t633: F) -> (F, F, F) {
    let t27671 = t27582 + t27620 + t27645 + t27669;
    let t27673 = t8010 * t1636;
    let t27676 = t27671 * t633 + F::cast_from(4.0_f64) * t27673 * t4480 - t27490 + t27493 - t27496 + t27497 - t27500 + t27505 - t27508 - t27511 + t27554;
    (t27671, t27673, t27676)
}
