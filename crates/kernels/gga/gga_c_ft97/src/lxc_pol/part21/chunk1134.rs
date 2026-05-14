//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1134/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1134<F: Float>(t16011: F, t1737: F, t5570: F, t22572: F, t29486: F, t5569: F, t25752: F, t58180: F, t29482: F, t92642: F, t100634: F, t16169: F, t22767: F, t29554: F, t22632: F, t5611: F) -> (F, F, F, F, F, F, F, F) {
    let t115944 = t5570 * t1737 * t16011;
    let t115956 = t5569 * t22572 * t29486;
    let t115970 = t58180 * t25752;
    let t115973 = t29482 * t92642;
    let t115977 = t100634 * t1737 * t16169;
    let t115981 = t22767 * t29554;
    let t115984 = t22632 * t29554;
    let t115985 = t5611 * t115984;
    (t115944, t115956, t115970, t115973, t115977, t115981, t115984, t115985)
}
