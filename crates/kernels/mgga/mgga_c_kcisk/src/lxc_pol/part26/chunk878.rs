//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 878/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk878<F: Float>(t1308: F, t6148: F, t13893: F, t9: F, t403: F, t3951: F, t963: F, t13485: F, t6179: F, t3935: F, t13900: F, t2163: F, t1309: F, t1311: F, t3118: F, t6188: F) -> (F, F, F, F, F, F, F) {
    let t20097 = t6148 * t1308;
    let t20110 = t9 * t13893;
    let t20111 = t20110 * t403;
    let t20115 = t963 * t3951;
    let t20116 = t20115 * t403;
    let t20124 = t13485 * t6179;
    let t20126 = 0.11993859144118211475e-1 * t3935 * t20124;
    let t20127 = t13900 * t2163;
    let t20128 = t1309 * t20127;
    let t20149 = t3118 * t1311;
    let t20150 = t20149 * t6188;
    (t20097, t20111, t20116, t20126, t20128, t20149, t20150)
}
