//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 923/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk923<F: Float>(t16326: F, t4704: F, t2382: F, t4736: F, t4705: F, t6838: F, t10715: F, t4744: F, t6834: F, t1663: F, t4742: F, t10757: F, t2381: F, t10755: F, t5409: F, t6879: F) -> (F, F, F, F, F, F, F) {
    let t16328 = 4.0 * t4704 * t16326;
    let t16329 = t2382 * t4736;
    let t16331 = 2.0 * t4704 * t16329;
    let t16332 = t6838 * t4705;
    let t16334 = 0.96490945932906628932e2 * t10715 * t16332;
    let t16335 = t6834 * t4744;
    let t16336 = t16335 * t1663;
    let t16338 = 0.32163648644302209644e2 * t4742 * t16336;
    let t16339 = t6838 * t4736;
    let t16341 = 0.16081824322151104822e2 * t4742 * t16339;
    let t16342 = t2381 * t10757;
    let t16343 = t16342 * t4705;
    let t16345 = 0.51725014705706168417e3 * t10755 * t16343;
    let t16346 = t6879 * t5409;
    (t16328, t16331, t16334, t16338, t16341, t16345, t16346)
}
