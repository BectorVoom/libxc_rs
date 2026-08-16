//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 649/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk649<F: Float>(t10887: F, t2021: F, t2975: F, t7372: F, t1: F, t10686: F, t787: F, t2465: F, t2949: F, t2464: F, t825: F, t8516: F, t959: F) -> (F, F, F, F, F, F) {
    let t10888 = F::cast_from(0.14896037479937677779e-1_f64) * t10887;
    let t10889 = t2021 * t2975;
    let t10890 = t10889 * t7372;
    let t10891 = F::cast_from(0.14896037479937677779e-1_f64) * t10890;
    let t10892 = t10686 * t1;
    let t10893 = t787 * t10892;
    let t10896 = t2465 * t2949;
    let t10897 = t2464 * t10896;
    let t10898 = t825 * t10897;
    let t10899 = F::cast_from(0.42603251059911944084e-1_f64) * t10898;
    let t10900 = t8516 * t959;
    (t10888, t10891, t10892, t10893, t10899, t10900)
}
