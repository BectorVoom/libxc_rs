//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 367/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk367<F: Float>(t1336: F, t789: F, t796: F, t1329: F, t226: F, t238: F, t242: F, t1331: F, t794: F, t804: F) -> (F, F, F, F, F) {
    let t1337 = t789 * t1336;
    let t1340 = t796 * t1336;
    let t1342 = t226 * t1329;
    let t1344 = t238 * t242 * t1342;
    let t1346 = F::cast_from(0.1898925e1_f64) * t1337 - t794 + F::cast_from(0.8969e0_f64) * t1331 + F::cast_from(0.3071625e0_f64) * t1340 - t804 + F::cast_from(0.24647e0_f64) * t1344;
    (t1337, t1340, t1342, t1344, t1346)
}
