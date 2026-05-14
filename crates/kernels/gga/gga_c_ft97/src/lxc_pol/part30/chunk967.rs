//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 967/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk967<F: Float>(t152767: F, t7512: F, t7638: F, t7641: F, t1234: F, t193: F, t33953: F, t6308: F, t852: F, t35828: F, t684: F, t43381: F, t446: F, t152717: F, t10248: F, t152722: F) -> (F, F, F, F, F, F) {
    let t152792 = t7638 * t7512 * t7641 * t152767;
    let t152797 = t6308 * t193 * t852 * t33953 * t1234;
    let t152799 = t35828 * t684;
    let t152801 = t446 * t43381 * t152799;
    let t152804 = t446 * t43381 * t152717;
    let t152807 = t446 * t10248 * t152722;
    (t152792, t152797, t152799, t152801, t152804, t152807)
}
