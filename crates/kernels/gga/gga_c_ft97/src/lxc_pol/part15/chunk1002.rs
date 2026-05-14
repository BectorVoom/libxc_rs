//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1002/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1002<F: Float>(t5064: F, t68528: F, t1168: F, t80522: F, t10079: F, t10157: F, t14127: F, t18675: F, t1901: F, t21416: F, t21499: F, t242: F, t2574: F, t2606: F, t265: F, t3885: F, t3891: F, t446: F, t4934: F, t5053: F, t5181: F, t51882: F, t65313: F, t67881: F, t762: F, t88079: F, t88098: F, t88114: F, t9808: F) -> (F, F, F) {
    let t89147 = t68528 * t5064;
    let t89179 = t80522 * t1168;
    let t89187 = 4.0 * t446 * t242 * t89147 + 4.0 * t446 * t2574 * t5181 * t4934 - 112.0 / 81.0 * t51882 - 8.0 / 9.0 * t1901 * t3891 * t65313 * t88079 - 16.0 / 27.0 * t67881 + 8.0 * t446 * t10157 * t762 * t21416 * t1168 - 12.0 * t446 * t10157 * t265 * t4934 * t5053 + 8.0 / 3.0 * t1901 * t2606 * t9808 * t88098 + 8.0 / 3.0 * t1901 * t10079 * t3885 * t88114 - 4.0 / 3.0 * t446 * t242 * t89179 - 8.0 * t1901 * t14127 * t18675 * t21499;
    (t89147, t89179, t89187)
}
