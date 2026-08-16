//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 889/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk889<F: Float>(t17753: F, t2594: F, t3281: F, t1091: F, t3821: F, t2354: F, t446: F, t4965: F, t713: F, t9744: F, t4917: F, t9577: F) -> (F, F, F, F, F, F) {
    let t17754 = t2594 * t17753;
    let t17755 = t3281 * t17754;
    let t17757 = t1091 * t3821;
    let t17758 = t2354 * t17757;
    let t17759 = t446 * t17758;
    let t17761 = t4965 * t713;
    let t17762 = t9744 * t17761;
    let t17763 = t446 * t17762;
    let t17765 = t9577 * t4917;
    (t17755, t17757, t17759, t17761, t17763, t17765)
}
