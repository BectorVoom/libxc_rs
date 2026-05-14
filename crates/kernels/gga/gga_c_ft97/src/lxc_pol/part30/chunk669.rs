//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 669/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk669<F: Float>(t3758: F, t6783: F, t17839: F, t218: F, t3762: F, t1100: F, t695: F, t52: F, t709: F, t7457: F, t1613: F, t213: F, t7464: F) -> (F, F, F, F, F, F) {
    let t33383 = t3758 * t6783;
    let t33384 = t17839 * t218;
    let t33385 = t33384 * t3762;
    let t33388 = t1100 * t695;
    let t33390 = t52 * t7457 * t709;
    let t33394 = t1613 * t213 * t7464;
    (t33383, t33384, t33385, t33388, t33390, t33394)
}
