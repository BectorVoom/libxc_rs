//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 985/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk985<F: Float>(t32063: F, t32888: F, t32890: F, t32938: F, t376: F, t5890: F, t32917: F, t1369: F, t32952: F, t1637: F, t7374: F, t7378: F) -> (F, F, F, F, F, F, F) {
    let t139257 = t32888 * t32063 * t32890;
    let t139275 = t5890 * t376 * t32938;
    let t139278 = t5890 * t376 * t32917;
    let t139312 = t1369 * t376 * t32952;
    let t139320 = t1369 * t1637 * t7374;
    let t139321 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t139320;
    let t139323 = t1369 * t1637 * t7378;
    (t139257, t139275, t139278, t139312, t139320, t139321, t139323)
}
