//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1160/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1160<F: Float>(t24378: F, t25070: F, t28599: F, t111837: F, t2691: F, t218: F, t4088: F, t703: F, t1471: F, t19116: F, t28567: F, t108596: F, t28552: F, t112223: F, t19038: F, t287: F, t6793: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112266 = t25070 * t24378 * t28599;
    let t112268 = t2691 * t111837;
    let t112282 = t218 * t4088;
    let t112295 = t703 * t4088;
    let t112300 = t19116 * t1471;
    let t112358 = 0.22226000364197530866e-1 * t25070 * t24378 * t28567;
    let t112365 = 0.22226000364197530866e-1 * t28552 * t108596;
    let t112366 = t19038 * t112223;
    let t112367 = t6793 * t287;
    (t112266, t112268, t112282, t112295, t112300, t112358, t112365, t112366, t112367)
}
