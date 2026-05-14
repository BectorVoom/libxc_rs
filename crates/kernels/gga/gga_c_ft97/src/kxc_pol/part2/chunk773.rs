//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 773/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk773<F: Float>(t13350: F, t13697: F, t734: F, t91: F, t2601: F, t3699: F, t2354: F, t446: F, t3690: F, t9744: F, t1131: F, t2373: F, t7514: F, t193: F, t89: F, t1087: F, t9733: F) -> (F, F, F, F, F, F, F) {
    let t13698 = t13350 + t13697;
    let t13700 = t91 * t734 * t13698;
    let t13702 = t3699 * t2601;
    let t13703 = t2354 * t13702;
    let t13704 = t446 * t13703;
    let t13706 = t3690 * t2601;
    let t13707 = t9744 * t13706;
    let t13708 = t446 * t13707;
    let t13717 = t7514 * t1131 * t2373;
    let t13719 = t89 * t193 * t13717;
    let t13722 = t89 * t9733 * t1087;
    (t13700, t13702, t13704, t13706, t13708, t13719, t13722)
}
