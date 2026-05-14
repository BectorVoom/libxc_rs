//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 620/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk620<F: Float>(t13746: F, t13672: F, t676: F, t27: F, t89: F, t375: F, t3822: F, t2601: F, t3712: F) -> (F, F, F, F, F) {
    let t13747 = 4.0 / 9.0 * t13746;
    let t13748 = t676 * t13672;
    let t13750 = t89 * t27 * t13748;
    let t13753 = t89 * t375 * t3822;
    let t13754 = 2.0 / 9.0 * t13753;
    let t13757 = t3712 * t2601;
    (t13747, t13750, t13753, t13754, t13757)
}
