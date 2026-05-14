//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1286/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1286<F: Float>(t32388: F, t9860: F, t20160: F, t33831: F, t9536: F, t33836: F, t109514: F, t33915: F, t33910: F, t32439: F, t12261: F, t9854: F, t2737: F, t113642: F, t33873: F, t9524: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t114983 = t9860 * t32388;
    let t114991 = t9536 * t20160 * t33831;
    let t114995 = t20160 * t33836;
    let t114997 = 0.34722222222222222222e-2 * t9536 * t114995;
    let t115001 = 0.23148148148148148148e-2 * t9536 * t109514 * t33915;
    let t115002 = t109514 * t33910;
    let t115004 = 0.44675925925925925926e-3 * t32439 * t115002;
    let t115026 = t12261 * t9854;
    let t115027 = t2737 * t115026;
    let t115036 = 0.15476481481481481481e-2 * t113642;
    let t115058 = 0.34722222222222222222e-2 * t9524 * t33873;
    (t114983, t114991, t114995, t114997, t115001, t115002, t115004, t115026, t115027, t115036, t115058)
}
