//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 706/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk706<F: Float>(t11849: F, t959: F, t11823: F, t7785: F, t2321: F, t3701: F, t882: F, t11986: F, t2325: F, t883: F, t12446: F, t12450: F) -> (F, F, F, F, F, F, F, F) {
    let t13702 = t11849 * t959;
    let t13703 = F::cast_from(0.14896037479937677779e-1_f64) * t13702;
    let t13704 = t11823 * t7785;
    let t13725 = t3701 * t2321;
    let t13726 = t882 * t13725;
    let t13740 = t2325 * t883 * t11986;
    let t13741 = t882 * t13740;
    let t13775 = F::cast_from(0.63904876589867916128e-1_f64) * t12446;
    let t13776 = F::cast_from(0.63904876589867916128e-1_f64) * t12450;
    (t13703, t13704, t13725, t13726, t13740, t13741, t13775, t13776)
}
