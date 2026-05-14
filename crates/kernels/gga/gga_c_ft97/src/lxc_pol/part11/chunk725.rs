//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 725/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk725<F: Float>(t2739: F, t875: F, t840: F, t871: F, t2801: F, t824: F, t1882: F, t2869: F, t8232: F, t837: F, t10675: F, t10678: F, t10681: F, t10685: F, t10690: F, t10693: F, t10700: F, t10705: F, t10709: F, t10714: F, t10719: F, t1901: F, t446: F) -> (F, F, F, F, F) {
    let t10722 = t2739 * t875;
    let t10724 = t840 * t871 * t10722;
    let t10726 = t2801 * t824;
    let t10728 = t840 * t871 * t10726;
    let t10730 = t1882 * t2869;
    let t10732 = t8232 * t837;
    let t10734 = 2.0 / 3.0 * t446 * t10675 + t10678 / 9.0 - t446 * t10681 - 2.0 * t446 * t10685 + 2.0 * t446 * t10690 + 2.0 / 3.0 * t10693 - 2.0 * t446 * t10700 - 2.0 / 3.0 * t1901 * t10705 + 2.0 * t446 * t10709 + 2.0 * t446 * t10714 + 2.0 * t446 * t10719 + t446 * t10724 + t446 * t10728 - 2.0 / 3.0 * t10730 - 4.0 / 27.0 * t10732;
    (t10722, t10724, t10726, t10728, t10734)
}
