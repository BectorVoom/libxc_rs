//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 901/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk901<F: Float>(t10886: F, t9824: F, t2021: F, t2975: F, t7372: F, t2465: F, t2949: F, t2464: F, t825: F, t8516: F, t959: F, t10847: F, t7573: F, t7572: F, t10820: F, t326: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10887 = t10886 * t9824;
    let t10888 = 0.14896037479937677779e-1 * t10887;
    let t10889 = t2021 * t2975;
    let t10890 = t10889 * t7372;
    let t10891 = 0.14896037479937677779e-1 * t10890;
    let t10896 = t2465 * t2949;
    let t10897 = t2464 * t10896;
    let t10898 = t825 * t10897;
    let t10899 = 0.42603251059911944084e-1 * t10898;
    let t10900 = t8516 * t959;
    let t10901 = 0.14896037479937677779e-1 * t10900;
    let t10903 = t7573 * t10847;
    let t10905 = 0.69017266717057349418e1 * t7572 * t10903;
    let t10906 = t326 * t10820;
    (t10888, t10889, t10891, t10896, t10897, t10899, t10901, t10903, t10905, t10906)
}
