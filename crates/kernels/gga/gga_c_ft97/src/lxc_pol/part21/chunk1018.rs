//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1018/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1018<F: Float>(t1637: F, t5706: F, t89: F, t5733: F, t8232: F, t5724: F, t23339: F, t47660: F, t1786: F, t5710: F, t1851: F, t5743: F, t5657: F, t5646: F, t1334: F, t3281: F) -> (F, F, F, F, F, F, F, F, F) {
    let t91629 = t89 * t1637 * t5706;
    let t91705 = t8232 * t5733;
    let t91718 = t8232 * t5724;
    let t91739 = t47660 * t23339;
    let t91771 = t1786 * t5710;
    let t91817 = t1851 * t5743;
    let t91895 = t8232 * t5657;
    let t91897 = t8232 * t5646;
    let t92024 = 28.0 / 81.0 * t3281 * t1334;
    (t91629, t91705, t91718, t91739, t91771, t91817, t91895, t91897, t92024)
}
