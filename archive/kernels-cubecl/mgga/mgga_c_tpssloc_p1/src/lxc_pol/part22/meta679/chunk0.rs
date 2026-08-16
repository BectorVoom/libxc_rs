//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2241/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2241<F: Float>(t1041: F, t10868: F, t248: F, t5681: F, t13965: F, t4641: F, t17659: F, t3048: F, t14207: F, t4630: F, t13969: F, t17717: F, t3039: F) -> (F, F, F, F, F) {
    let t62137 = t1041 * t248 * t10868 * t5681;
    let t62148 = t4641 * t13965;
    let t62150 = t3048 * t17659;
    let t62152 = t14207 * t4630;
    let t62164 = t3039 * t13969 * t17717;
    (t62137, t62148, t62150, t62152, t62164)
}
