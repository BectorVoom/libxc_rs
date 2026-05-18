//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1047/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1047<F: Float>(t18: F, t3281: F, t33460: F, t9770: F, t35516: F, t668: F, t2354: F, t446: F, t505: F, t150064: F, t150034: F, t41879: F) -> (F, F, F, F) {
    let t150966 = t3281 * t9770 * t33460 * t18;
    let t150968 = t35516 * t668;
    let t150971 = t446 * t2354 * t150968 * t505;
    let t150974 = t446 * t9770 * t150064;
    let t150977 = t446 * t41879 * t150034;
    (t150966, t150971, t150974, t150977)
}
