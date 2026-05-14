//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1262/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1262<F: Float>(t119567: F, t23667: F, t27142: F, t30244: F, t379: F, t5899: F, t30220: F, t95053: F, t16950: F, t23892: F, t16150: F, t23909: F, t23671: F, t16169: F, t23657: F, t27086: F, t925: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t119569 = t27142 * t23667 * t119567;
    let t119571 = t30244 * t379;
    let t119573 = t5899 * t23667 * t119571;
    let t119575 = t95053 * t30220;
    let t119576 = t119575 / 18.0;
    let t119577 = t23892 * t16950;
    let t119579 = t5899 * t23667 * t119577;
    let t119581 = t23909 * t16150;
    let t119583 = t5899 * t23671 * t119581;
    let t119584 = t23892 * t16169;
    let t119586 = t27142 * t23671 * t119584;
    let t119590 = t23657 * t23671 * t27086 * t925;
    (t119569, t119571, t119573, t119575, t119576, t119577, t119579, t119581, t119583, t119584, t119586, t119590)
}
