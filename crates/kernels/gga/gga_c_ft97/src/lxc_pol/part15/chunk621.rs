//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 621/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk621<F: Float>(t1160: F, t737: F, t2567: F, t668: F, t1144: F, t8232: F, t2372: F, t255: F, t1131: F, t761: F, t13722: F, t13739: F) -> (F, F, F, F, F, F, F, F) {
    let t13839 = t737 * t1160;
    let t13857 = t2567 * t668;
    let t13872 = t8232 * t1144;
    let t13885 = t2372 * t255;
    let t13886 = t761 * t1131;
    let t13927 = t1160 * t2567;
    let t13976 = F::new(4.0) / F::new(27.0) * t13722;
    let t13981 = F::new(4.0) / F::new(9.0) * t13739;
    (t13839, t13857, t13872, t13885, t13886, t13927, t13976, t13981)
}
