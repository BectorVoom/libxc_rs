//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 606/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk606<F: Float>(t731: F, t8232: F, t768: F, t9735: F, t9701: F, t251: F, t631: F, t675: F, t7242: F, t898: F, t2371: F, t665: F, t2: F, t740: F, t8282: F, t9802: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9863 = 4.0 / 27.0 * t9735;
    let t9867 = 4.0 / 9.0 * t9701;
    let t9890 = 1.0 / t251 / t631 / t898 / t675 / t7242 / 4.0;
    let t9895 = t665 * t2371;
    let t9896 = t9895 * t2;
    let t9907 = t8282 * t740;
    let t9916 = t9802 * t2;
    (t9822, t9824, t9863, t9867, t9890, t9895, t9896, t9907, t9916)
}
