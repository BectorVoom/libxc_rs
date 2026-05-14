//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 799/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk799<F: Float>(t297: F, t9575: F, t2070: F, t559: F, t2351: F, t296: F, t2071: F, t2752: F, t2714: F, t6221: F, t2173: F, t9427: F, t6204: F) -> (F, F, F, F, F, F, F) {
    let t9576 = t297 * t9575;
    let t9783 = t2070 * t559;
    let t9786 = t296 * t2351;
    let t9789 = t2071 * t2752;
    let t9792 = t6221 * t2714;
    let t9795 = t9427 * t2173;
    let t9796 = t6204 * t9795;
    (t9576, t9783, t9786, t9789, t9792, t9795, t9796)
}
