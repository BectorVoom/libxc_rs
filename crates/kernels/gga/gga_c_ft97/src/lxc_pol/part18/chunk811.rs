//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 811/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk811<F: Float>(t22563: F, t7983: F, t25: F, t5532: F, t3066: F, t11352: F, t5513: F, t1690: F, t1696: F, t5552: F, t1689: F, t39: F, t77: F, t1608: F, t1613: F, t5551: F) -> (F, F, F, F, F, F, F, F) {
    let t22715 = t7983 * t22563;
    let t22718 = t5532 * t25;
    let t22719 = t22718 * t3066;
    let t22722 = t5513 * t11352;
    let t22726 = t1690 * t5552 * t1696;
    let t22735 = t77 * t39 * t1689;
    let t22736 = t1608 * t22735;
    let t22737 = t1613 * t5551;
    (t22715, t22718, t22719, t22722, t22726, t22735, t22736, t22737)
}
