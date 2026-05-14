//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 828/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk828<F: Float>(t18729: F, t684: F, t2599: F, t14159: F, t3898: F, t13839: F, t3870: F, t5147: F, t761: F, t2606: F, t5134: F, t681: F, t89: F, t1168: F, t3972: F, t2568: F) -> (F, F, F, F, F, F) {
    let t18730 = t18729 * t684;
    let t18731 = t2599 * t18730;
    let t18734 = t14159 * t3898;
    let t18737 = t13839 * t3870;
    let t18740 = t761 * t5147;
    let t18741 = t18740 * t684;
    let t18742 = t2606 * t18741;
    let t18746 = t89 * t681 * t5134;
    let t18749 = t1168 * t3972;
    let t18750 = t2568 * t18749;
    (t18731, t18734, t18737, t18742, t18746, t18750)
}
