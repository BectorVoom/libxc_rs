//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 467/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk467<F: Float>(t236: F, t6: F, t51: F, t3771: F, t1614: F, t213: F, t1109: F, t3762: F) -> (F, F, F, F, F) {
    let t3772 = t236 * t6;
    let t3773 = t3772 * t51;
    let t3774 = t3771 * t3773;
    let t3775 = t1614 * t213;
    let t3776 = t3775 * t1109;
    let t3777 = t3776 * t3762;
    let t3780 = t213 * t1109;
    (t3774, t3775, t3776, t3777, t3780)
}
