//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2196/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2196<F: Float>(t5572: F, t9541: F, t5527: F, t828: F, t5611: F, t5624: F, t9601: F, t1512: F, t47092: F, t13257: F, t4166: F, t4184: F) -> (F, F, F, F, F, F) {
    let t58550 = t9541 * t5572;
    let t58557 = t5527 * t828;
    let t58569 = t5611 * t828;
    let t58574 = t9601 * t5624;
    let t58576 = t47092 * t1512;
    let t58616 = t4166 * t13257 * t4184;
    (t58550, t58557, t58569, t58574, t58576, t58616)
}
