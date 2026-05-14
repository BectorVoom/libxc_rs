//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 877/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk877<F: Float>(t13959: F, t6226: F, t13485: F, t6200: F, t3935: F, t1311: F, t963: F, t403: F, t1319: F, t3970: F, t6171: F, t3966: F, t2159: F, t3969: F, t3974: F, t6157: F) -> (F, F, F, F, F, F, F, F) {
    let t20002 = t13959 * t6226;
    let t20034 = t13485 * t6200;
    let t20036 = 0.11993859144118211475e-1 * t3935 * t20034;
    let t20052 = t963 * t1311;
    let t20053 = t20052 * t403;
    let t20067 = t20052 * t1319;
    let t20072 = t3970 * t6171;
    let t20075 = 0.11993859144118211475e-1 * t3966 * t6171;
    let t20084 = t2159 * t3969;
    let t20088 = 0.11993859144118211475e-1 * t6157 * t3974;
    (t20002, t20036, t20053, t20067, t20072, t20075, t20084, t20088)
}
