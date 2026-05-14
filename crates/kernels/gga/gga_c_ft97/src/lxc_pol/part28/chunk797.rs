//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 797/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk797<F: Float>(t33106: F, t33110: F, t33114: F, t34851: F, t34856: F, t34921: F, t34925: F, t34929: F, t34933: F, t34937: F, t34941: F, t34945: F, t35138: F, t143: F, t160: F, t1384: F, t6615: F) -> (F, F, F) {
    let t35148 = t33106 + t34851 / 18.0 + t34856 / 3.0 - t34921 / 6.0 - t33110 - 2.0 / 9.0 * t34925 - 2.0 * t34929 + 4.0 / 3.0 * t34933 + t33114 + t34937 / 9.0 + 2.0 / 3.0 * t34941 - t34945 / 3.0;
    let t35149 = t35138 + t35148;
    let t35151 = t143 * t35149 * t160;
    let t35155 = t6615 * t1384;
    (t35149, t35151, t35155)
}
