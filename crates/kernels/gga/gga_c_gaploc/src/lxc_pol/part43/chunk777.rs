//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 777/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk777<F: Float>(t2508: F, t28953: F, t9014: F, t1897: F, t2580: F, t28236: F, t2958: F, t1022: F, t6058: F, t28668: F, t5241: F, t43107: F, t7290: F, t1841: F, t7289: F, t40836: F) -> (F, F, F, F, F, F) {
    let t43185 = 0.18457262952341338281e0 * t2508 * t9014 * t28953;
    let t43189 = 0.15381052460284448567e-1 * t1897 * t2580 * t2958 * t28236;
    let t43191 = t6058 * t1022;
    let t43195 = 0.46143157380853345701e0 * t2508 * t43191 * t5241 * t28668;
    let t43199 = t7290 * t43107;
    let t43202 = 0.17090058289204942852e-2 * t1841 * t7289 * t43199;
    let t43207 = 0.64087718584518535698e-3 * t40836;
    (t43185, t43189, t43195, t43199, t43202, t43207)
}
