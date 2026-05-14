//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1311/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1311<F: Float>(t105353: F, t27072: F, t5899: F, t13043: F, t5916: F, t95344: F, t23657: F, t23671: F, t23900: F, t27081: F, t3424: F, t23667: F, t1651: F, t6630: F, t1643: F, t95340: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t105355 = t5899 * t27072 * t105353;
    let t105357 = t5916 * t13043;
    let t105359 = t5899 * t95344 * t105357;
    let t105362 = t23657 * t23671 * t23900 * t27081;
    let t105364 = t23900 * t3424;
    let t105366 = t5899 * t23667 * t105364;
    let t105368 = t6630 * t1651;
    let t105370 = t5899 * t23667 * t105368;
    let t105372 = t6630 * t1643;
    let t105374 = t5899 * t95340 * t105372;
    (t105355, t105357, t105359, t105362, t105364, t105366, t105368, t105370, t105372, t105374)
}
