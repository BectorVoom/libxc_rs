//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 876/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk876<F: Float>(t5379: F, t852: F, t1222: F, t4137: F, t3882: F, t5384: F, t5385: F, t1620: F, t3896: F, t1308: F, t3912: F, t1614: F, t3901: F, t3930: F, t3868: F, t5351: F) -> (F, F, F, F, F, F, F, F) {
    let t15192 = t852 * t5379;
    let t15196 = t4137 * t1222;
    let t15199 = t5384 * t5385 * t3882;
    let t15201 = t3896 * t1620;
    let t15204 = t1308 * t3912;
    let t15206 = t3901 * t1614;
    let t15208 = t3930 * t1614;
    let t15210 = t3868 * t5351;
    (t15192, t15196, t15199, t15201, t15204, t15206, t15208, t15210)
}
