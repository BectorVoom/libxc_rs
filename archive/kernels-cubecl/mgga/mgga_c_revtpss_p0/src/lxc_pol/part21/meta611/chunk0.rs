//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2362/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2362<F: Float>(t2523: F, t9318: F, t2596: F, t746: F, t9385: F, t760: F, t186: F, t2698: F, t685: F, t755: F, t10558: F, t177: F, t762: F) -> (F, F, F, F, F, F) {
    let t40094 = t2523 * t9318;
    let t40097 = t2596 * t9385 * t746;
    let t40099 = F::cast_from(0.46785788981077169656e1_f64) * t760 * t40097;
    let t40101 = t685 * t2698 * t186;
    let t40103 = F::cast_from(0.18989649058080861537e-2_f64) * t755 * t40101;
    let t40108 = t10558 * t177 * t762;
    (t40094, t40097, t40099, t40101, t40103, t40108)
}
