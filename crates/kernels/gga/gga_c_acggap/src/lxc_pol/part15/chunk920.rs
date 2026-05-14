//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 920/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk920<F: Float>(t1423: F, t7746: F, t31752: F, t1507: F, t2020: F, t30120: F, t8793: F, t8948: F, t7839: F, t8787: F, t30689: F, t5286: F, t1181: F, t22275: F, t604: F, t7493: F) -> (F, F, F, F, F, F, F, F) {
    let t36139 = t7746 * t1423;
    let t36141 = 0.26416397523267487738e-1 * t31752;
    let t36151 = t2020 * t1507;
    let t36156 = t30120 * t8793;
    let t36162 = t30120 * t8948;
    let t36175 = t7839 * t8787;
    let t36177 = t30689 * t5286;
    let t36194 = t7493 * t1181 * t604 * t22275;
    (t36139, t36141, t36151, t36156, t36162, t36175, t36177, t36194)
}
