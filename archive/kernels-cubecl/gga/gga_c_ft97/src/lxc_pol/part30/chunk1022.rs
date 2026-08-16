//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1022/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1022<F: Float>(t230: F, t3817: F, t420: F, t226: F, t35371: F, t1127: F, t140919: F, t3762: F, t1613: F, t213: F, t6793: F, t27729: F, t9: F) -> (F, F, F, F, F) {
    let t150496 = t420 * t230 * t3817;
    let t150500 = t35371 * t226;
    let t150511 = t140919 * t1127;
    let t150512 = t150511 * t3762;
    let t150516 = t1613 * t6793 * t213;
    let t150517 = t150516 * t3762;
    let t150522 = t27729 * t9;
    (t150496, t150500, t150512, t150517, t150522)
}
