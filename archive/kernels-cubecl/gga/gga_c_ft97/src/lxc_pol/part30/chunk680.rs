//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 680/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk680<F: Float>(t28538: F, t28767: F, t28807: F, t28840: F, t312: F, t10688: F, t7114: F, t1248: F, t6386: F, t2843: F, t15128: F, t6374: F) -> (F, F, F, F, F, F) {
    let t28842 = t28538 + t28767 + t28807 + t28840;
    let t28843 = t28842 * t312;
    let t28845 = t10688 * t7114;
    let t28847 = t6386 * t1248;
    let t28848 = t2843 * t28847;
    let t28850 = t15128 * t6374;
    (t28842, t28843, t28845, t28847, t28848, t28850)
}
