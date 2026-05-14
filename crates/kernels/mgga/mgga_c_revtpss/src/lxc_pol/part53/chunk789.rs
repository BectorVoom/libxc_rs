//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 789/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk789<F: Float>(t1234: F, t7623: F, t1210: F, t8945: F, t487: F, t7642: F, t1269: F, t3140: F, t1276: F, t2148: F, t1243: F, t8939: F, t2149: F, t7627: F, t1032: F, t12626: F, t2147: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26880 = t1234 * t7623;
    let t26889 = t1210 * t8945;
    let t26894 = t7642 * t487;
    let t26895 = t26894 * t8945;
    let t26916 = t1269 * t3140;
    let t26918 = t2148 * t26916 * t1276;
    let t26921 = t8939 * t1243;
    let t26922 = t2149 * t26921;
    let t26931 = t1243 * t7627;
    let t26936 = t1269 * t1032;
    let t26937 = t2148 * t26936;
    let t26948 = t2147 * t12626;
    (t26880, t26889, t26895, t26918, t26922, t26931, t26936, t26937, t26948)
}
