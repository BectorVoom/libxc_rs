//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 902/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk902<F: Float>(t1976: F, t5015: F, t7160: F, t3046: F, t7143: F, t1032: F, t1678: F, t7150: F, t4742: F, t7145: F, t1695: F, t7135: F) -> (F, F, F, F, F, F) {
    let t27411 = t1976 * t5015;
    let t27412 = t7160 * t27411;
    let t27415 = t3046 * t7143;
    let t27418 = t1678 * t1032;
    let t27419 = t7150 * t27418;
    let t27422 = t1976 * t4742;
    let t27423 = t7145 * t27422;
    let t27426 = t7135 * t1695;
    (t27412, t27415, t27418, t27419, t27423, t27426)
}
