//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 675/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk675<F: Float>(t768: F, t8232: F, t1882: F, t2563: F, t2559: F, t265: F, t724: F, t9587: F, t2594: F, t9578: F, t2526: F, t766: F, t2568: F, t242: F, t1901: F, t446: F, t9788: F, t9794: F, t9799: F, t9805: F, t9810: F, t9813: F, t9816: F, t9819: F, t9822: F) -> (F, F, F, F, F, F) {
    let t9824 = t8232 * t768;
    let t9826 = t1882 * t2563;
    let t9828 = t1882 * t2559;
    let t9831 = t724 * t265 * t9587;
    let t9835 = t2594 * t265 * t9578;
    let t9838 = t766 * t2526;
    let t9839 = t2568 * t9838;
    let t9840 = t242 * t9839;
    let t9843 = 2.0 / 3.0 * t1901 * t9788 - 2.0 / 3.0 * t1901 * t9794 - 2.0 / 3.0 * t1901 * t9799 + 2.0 / 9.0 * t1901 * t9805 + 2.0 / 9.0 * t1901 * t9810 + 2.0 / 3.0 * t9813 - t446 * t9816 - t446 * t9819 / 3.0 - 4.0 / 9.0 * t9822 - 4.0 / 9.0 * t9824 + t9826 / 3.0 + 2.0 / 3.0 * t9828 - 2.0 / 3.0 * t446 * t9831 + 4.0 / 9.0 * t446 * t9835 + 2.0 * t446 * t9840;
    (t9831, t9835, t9838, t9839, t9840, t9843)
}
