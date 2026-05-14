//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 997/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk997<F: Float>(t2735: F, t820: F, t2719: F, t10292: F, t10296: F, t10297: F, t10358: F, t10359: F, t10363: F, t10365: F, t10369: F, t10384: F, t14763: F, t2688: F, t2691: F, t2692: F, t2724: F, t2725: F, t2726: F, t2727: F, t2736: F, t284: F, t285: F, t291: F, t4061: F, t4113: F, t43586: F, t43587: F, t43595: F, t43626: F, t43639: F, t43702: F, t43766: F, t800: F, t811: F, t816: F, t817: F) -> (F,) {
    let t43777 = t820 * t2735;
    let t43781 = t2719 * t2719;
    let t43789 = 24.0 * t285 * t43586 * t43587 - 36.0 * t4113 * t10363 * t2726 * t2735 + 6.0 * t285 * t2725 * t43595 + 8.0 * t4113 * t10369 * t10384 - 12.0 * t2691 * t10296 * t2735 + 24.0 * t2688 * t2727 + 24.0 * t2691 * t2724 * t2719 * t2726 - 8.0 * t2691 * t816 * t10358 * t820 - t285 * t817 * (t43626 + t43639) - 8.0 * t2691 * t2692 * t10384 + 2.0 * t800 * t291 * (t43702 + t43766) - 48.0 * t2691 * t10363 * t811 * t10365 - 24.0 * t14763 * t10297 + 48.0 * t2691 * t10292 * t43777 + 6.0 * t43781 * t284 * t291 + 8.0 * t4061 * t10359 - 12.0 * t2688 * t2736;
    (t43789,)
}
