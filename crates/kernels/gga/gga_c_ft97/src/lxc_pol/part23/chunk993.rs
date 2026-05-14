//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 993/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk993<F: Float>(t1091: F, t27652: F, t27660: F, t27659: F, t4978: F, t6036: F, t30607: F, t6023: F, t1113: F, t1127: F, t231: F, t1614: F, t24389: F, t17836: F, t21145: F, t6758: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30617 = t27652 * t1091;
    let t30621 = t27660 * t1091;
    let t30622 = t27659 * t30621;
    let t30625 = t6036 * t4978;
    let t30632 = t6023 * t30607;
    let t30635 = t1113 * t1127;
    let t30636 = t231 * t30635;
    let t30640 = t24389 * t1614;
    let t30641 = t17836 * t30640;
    let t30642 = t6758 * t21145;
    (t30617, t30621, t30622, t30625, t30632, t30635, t30636, t30641, t30642)
}
