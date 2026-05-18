//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 707/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk707<F: Float>(t27215: F, t3188: F, t12709: F, t1384: F, t1557: F, t12714: F, t1882: F, t6636: F, t3429: F, t5916: F, t9144: F, t13208: F, t27068: F) -> (F, F, F, F, F, F, F, F) {
    let t27216 = t27215 * t3188;
    let t27217 = t12709 * t27216;
    let t27220 = t1384 * t1557;
    let t27221 = t27220 * t3188;
    let t27222 = t12714 * t27221;
    let t27226 = t1882 * t6636;
    let t27228 = t5916 * t3429;
    let t27229 = t9144 * t27228;
    let t27232 = t13208 * t27068;
    (t27216, t27217, t27221, t27222, t27226, t27228, t27229, t27232)
}
