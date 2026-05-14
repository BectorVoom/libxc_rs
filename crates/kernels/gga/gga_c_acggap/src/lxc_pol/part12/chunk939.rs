//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 939/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk939<F: Float>(t301: F, t4256: F, t7450: F, t8539: F, t2030: F, t372: F, t4262: F, t17752: F, t8923: F, t1016: F, t2060: F, t361: F, t8928: F, t20559: F, t7502: F, t15695: F, t8915: F) -> (F, F, F, F, F, F) {
    let t34909 = t7450 * t4256 * t8539 * t301;
    let t34913 = t2030 * t4262 * t8539 * t372;
    let t34916 = t2030 * t17752 * t8923;
    let t34920 = t2060 * t361 * t1016 * t8928;
    let t34923 = t2030 * t20559 * t7502;
    let t34926 = t7450 * t15695 * t8915;
    (t34909, t34913, t34916, t34920, t34923, t34926)
}
