//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1031/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1031<F: Float>(t27326: F, t1299: F, t1640: F, t2233: F, t4121: F, t541: F, t4125: F, t303: F, t1014: F, t7932: F, t7935: F, t12231: F, t1598: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27327 = t27326 / F::new(16.0);
    let t27328 = t1299 * t1640;
    let t27329 = t2233 * t27328;
    let t27330 = t27329 / F::new(8.0);
    let t27331 = t541 * t4121;
    let t27332 = t27331 * t4125;
    let t27333 = t303 * t27332;
    let t27335 = t1014 * t7932;
    let t27337 = t1014 * t7935;
    let t27339 = t12231 * t1598;
    (t27327, t27328, t27330, t27331, t27332, t27333, t27335, t27337, t27339)
}
