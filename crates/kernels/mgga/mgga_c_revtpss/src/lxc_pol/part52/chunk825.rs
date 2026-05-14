//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 825/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk825<F: Float>(t1976: F, t5015: F, t7160: F, t3046: F, t7143: F, t1032: F, t1678: F, t7150: F, t4742: F, t7145: F, t1695: F, t7135: F, t1043: F, t1089: F, t7817: F, t7821: F) -> (F, F, F, F, F, F, F, F) {
    let t27411 = t1976 * t5015;
    let t27412 = t7160 * t27411;
    let t27415 = t3046 * t7143;
    let t27418 = t1678 * t1032;
    let t27419 = t7150 * t27418;
    let t27422 = t1976 * t4742;
    let t27423 = t7145 * t27422;
    let t27426 = t7135 * t1695;
    let t27427 = t7160 * t27426;
    let t27433 = t7817 * t1043 * t1089;
    let t27437 = t7821 * t1043 * t1089;
    (t27412, t27415, t27418, t27419, t27423, t27427, t27433, t27437)
}
