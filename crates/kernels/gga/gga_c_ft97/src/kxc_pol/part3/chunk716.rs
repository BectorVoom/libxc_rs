//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 716/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk716<F: Float>(t1152: F, t1771: F, t2345: F, t26: F, t2347: F, t743: F, t666: F, t2360: F, t1087: F, t89: F, t9733: F, t11401: F, t665: F) -> (F, F, F, F, F, F, F, F) {
    let t13680 = t1771 * t1152;
    let t13682 = t26 * t2345;
    let t13683 = t743 * t2347;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13722 = t89 * t9733 * t1087;
    let t13723 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t13722;
    let t13730 = t11401 * t665;
    (t13680, t13682, t13683, t13688, t13689, t13722, t13723, t13730)
}
