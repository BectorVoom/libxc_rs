//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 685/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk685<F: Float>(t1882: F, t2751: F, t869: F, t309: F, t2770: F, t871: F, t2869: F, t8232: F, t837: F, t877: F, t2834: F, t681: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t10693 = t1882 * t2751;
    let t10695 = t869 * t869;
    let t10696 = F::cast_from(1.0_f64) / t10695;
    let t10697 = t309 * t10696;
    let t10703 = t2770 * t871;
    let t10730 = t1882 * t2869;
    let t10732 = t8232 * t837;
    let t10735 = t8232 * t877;
    let t10745 = t89 * t681 * t2834;
    (t10693, t10697, t10703, t10730, t10732, t10735, t10745)
}
