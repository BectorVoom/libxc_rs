//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1213/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1213<F: Float>(t108817: F, t18497: F, t2446: F, t108446: F, t3766: F, t17965: F, t4075: F, t420: F, t6036: F, t1096: F, t17859: F, t108685: F, t6808: F, t6809: F, t24330: F, t30721: F, t6043: F) -> (F, F, F, F, F, F, F) {
    let t122827 = t108817 * t2446 * t18497;
    let t122830 = t3766 * t108446;
    let t122836 = t4075 * t17965;
    let t122840 = t420 * t6036;
    let t122841 = t1096 * t17859;
    let t122846 = t6808 * t108685 * t6809;
    let t122849 = t6043 * t24330 * t30721;
    (t122827, t122830, t122836, t122840, t122841, t122846, t122849)
}
