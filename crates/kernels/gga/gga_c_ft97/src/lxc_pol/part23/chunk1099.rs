//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1099/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1099<F: Float>(t1443: F, t9802: F, t6187: F, t668: F, t1451: F, t3281: F, t6148: F, t737: F, t24668: F, t53798: F, t2492: F, t38953: F, t6076: F, t6168: F, t8232: F, t6177: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97522 = t9802 * t1443;
    let t97537 = t6187 * t668;
    let t97629 = 28.0 / 81.0 * t3281 * t1451;
    let t97701 = t737 * t6148;
    let t97705 = t53798 * t24668;
    let t97733 = t2492 * t6148;
    let t97740 = t38953 * t6076;
    let t97770 = t8232 * t6168;
    let t97772 = t8232 * t6177;
    (t97522, t97537, t97629, t97701, t97705, t97733, t97740, t97770, t97772)
}
