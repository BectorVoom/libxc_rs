//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 999/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk999<F: Float>(t3526: F, t7312: F, t32888: F, t7239: F, t7369: F, t147656: F, t446: F, t9073: F, t1369: F, t148132: F, t28: F, t9236: F, t148451: F, t2112: F, t34854: F, t376: F) -> (F, F, F, F, F, F) {
    let t148517 = t7312 * t3526;
    let t148520 = t32888 * t7239 * t7369 * t148517;
    let t148523 = t446 * t9073 * t147656;
    let t148527 = t1369 * t28 * t9236 * t148132;
    let t148530 = t1369 * t28 * t2112 * t148451;
    let t148533 = t1369 * t376 * t34854;
    (t148517, t148520, t148523, t148527, t148530, t148533)
}
