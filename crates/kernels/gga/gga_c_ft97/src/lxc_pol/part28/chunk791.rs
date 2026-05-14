//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 791/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk791<F: Float>(t1017: F, t1389: F, t5778: F, t28: F, t32709: F, t6587: F, t32717: F, t9073: F, t925: F, t1058: F, t7313: F, t1384: F, t6718: F, t2179: F, t144: F, t23997: F, t6708: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t35010 = t1389 * t1017;
    let t35011 = t5778 * t35010;
    let t35012 = t28 * t35011;
    let t35015 = t32709 * t6587;
    let t35016 = t28 * t35015;
    let t35022 = t9073 * t32717 * t925;
    let t35027 = t7313 * t1058;
    let t35028 = t28 * t35027;
    let t35033 = t1384 * t6718;
    let t35034 = t2179 * t35033;
    let t35035 = t144 * t35034;
    let t35038 = t23997 * t6708;
    (t35010, t35011, t35012, t35015, t35016, t35022, t35027, t35028, t35033, t35034, t35035, t35038)
}
