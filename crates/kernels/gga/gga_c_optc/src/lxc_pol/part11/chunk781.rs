//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 781/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk781<F: Float>(t1129: F, t5417: F, t2367: F, t5403: F, t1150: F, t1156: F, t5398: F, t3217: F, t2586: F, t5388: F, t1170: F, t1179: F, t15597: F, t5393: F, t3212: F, t1128: F, t5412: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15978 = t5417 * t1129;
    let t15980 = t2367 * t5403;
    let t15981 = t1150 * t15980;
    let t15983 = t1156 * t5398;
    let t15984 = t3217 * t15983;
    let t15986 = t2586 * t5388;
    let t15987 = t1170 * t15986;
    let t15996 = t1179 * t15597;
    let t16001 = t1156 * t5393;
    let t16002 = t3212 * t16001;
    let t16004 = t1128 * t5412;
    (t15978, t15980, t15981, t15983, t15984, t15986, t15987, t15996, t16001, t16002, t16004)
}
