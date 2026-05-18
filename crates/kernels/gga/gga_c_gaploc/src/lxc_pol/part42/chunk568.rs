//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 568/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk568<F: Float>(t10882: F, t2464: F, t2684: F, t787: F, t8788: F, t9824: F, t2021: F, t2975: F, t7372: F, t2465: F, t2949: F, t825: F) -> (F, F, F, F, F, F, F) {
    let t10883 = t2464 * t10882;
    let t10884 = t2684 * t10883;
    let t10885 = F::new(0.42603251059911944084e-1) * t10884;
    let t10886 = t787 * t8788;
    let t10887 = t10886 * t9824;
    let t10888 = F::new(0.14896037479937677779e-1) * t10887;
    let t10889 = t2021 * t2975;
    let t10890 = t10889 * t7372;
    let t10891 = F::new(0.14896037479937677779e-1) * t10890;
    let t10896 = t2465 * t2949;
    let t10897 = t2464 * t10896;
    let t10898 = t825 * t10897;
    (t10884, t10885, t10887, t10888, t10890, t10891, t10898)
}
