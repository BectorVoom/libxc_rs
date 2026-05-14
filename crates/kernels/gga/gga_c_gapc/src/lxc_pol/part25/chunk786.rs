//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 786/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk786<F: Float>(t9060: F, t9894: F, t2546: F, t3328: F, t2210: F, t2767: F, t3045: F, t7294: F, t3120: F, t3363: F, t1089: F, t122: F, t2153: F, t1034: F, t3364: F, t3368: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9895 = t9894 * t9060;
    let t9896 = t2546 * t3328;
    let t9897 = t2210 * t9896;
    let t9898 = t9895 * t9897;
    let t9901 = t7294 * t3045 * t2767;
    let t9903 = t3363 * t3120;
    let t9904 = t9903 * t1089;
    let t9906 = t2153 * t122;
    let t9907 = t9906 * t1034;
    let t9908 = t9907 * t1089;
    let t9910 = t3364 * t3368;
    (t9895, t9896, t9897, t9898, t9901, t9903, t9904, t9906, t9908, t9910)
}
