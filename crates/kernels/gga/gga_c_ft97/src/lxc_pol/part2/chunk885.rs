//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 885/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk885<F: Float>(t13739: F, t2459: F, t3717: F, t193: F, t89: F, t3718: F, t681: F, t13672: F, t676: F, t27: F, t375: F, t3822: F) -> (F, F, F, F, F, F) {
    let t13740 = F::new(4.0) / F::new(27.0) * t13739;
    let t13741 = t3717 * t2459;
    let t13743 = t89 * t193 * t13741;
    let t13746 = t89 * t681 * t3718;
    let t13747 = F::new(4.0) / F::new(9.0) * t13746;
    let t13748 = t676 * t13672;
    let t13750 = t89 * t27 * t13748;
    let t13753 = t89 * t375 * t3822;
    (t13740, t13743, t13746, t13747, t13750, t13753)
}
