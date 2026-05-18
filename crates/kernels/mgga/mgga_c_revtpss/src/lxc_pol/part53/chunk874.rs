//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 874/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk874<F: Float>(t1243: F, t8939: F, t2149: F, t7627: F, t1032: F, t1269: F, t2148: F, t12626: F, t2147: F, t7635: F, t13181: F, t473: F) -> (F, F, F, F, F, F) {
    let t26921 = t8939 * t1243;
    let t26922 = t2149 * t26921;
    let t26931 = t1243 * t7627;
    let t26936 = t1269 * t1032;
    let t26937 = t2148 * t26936;
    let t26948 = t2147 * t12626;
    let t26949 = t26948 * t7635;
    let t26969 = t13181 * t473;
    (t26922, t26931, t26936, t26937, t26949, t26969)
}
