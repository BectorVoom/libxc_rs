//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1271/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1271<F: Float>(t1969: F, t27142: F, t27165: F, t3052: F, t30213: F, t95053: F, t23649: F, t30183: F, t105462: F, t3188: F, t446: F, t119657: F, t9049: F, t23609: F, t28: F, t4753: F, t5842: F, t586: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119729 = t27142 * t1969 * t27165 * t3052;
    let t119731 = t95053 * t30213;
    let t119732 = t119731 / 3.0;
    let t119733 = t23649 * t30183;
    let t119734 = t119733 / 27.0;
    let t119735 = t105462 * t3188;
    let t119737 = t446 * t1969 * t119735;
    let t119740 = t446 * t9049 * t119657;
    let t119745 = t23609 * t28 * t586 * t5842 * t4753;
    (t119729, t119731, t119732, t119733, t119734, t119735, t119737, t119740, t119745)
}
