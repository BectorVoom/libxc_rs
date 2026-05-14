//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1173/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1173<F: Float>(t15885: F, t28: F, t5507: F, t89: F, t23054: F, t29638: F, t29661: F, t1564: F, t15772: F, t5674: F, t5675: F, t23057: F, t4454: F, t7793: F, t100285: F, t925: F) -> (F, F, F, F, F, F, F, F) {
    let t116708 = t89 * t28 * t5507 * t15885;
    let t116710 = t23054 * t29638;
    let t116711 = t116710 / 9.0;
    let t116712 = t23054 * t29661;
    let t116713 = t116712 / 18.0;
    let t116716 = t5674 * t1564 * t5675 * t15772;
    let t116720 = t5674 * t7793 * t23057 * t4454;
    let t116724 = t5674 * t1564 * t100285 * t925;
    (t116708, t116710, t116711, t116712, t116713, t116716, t116720, t116724)
}
