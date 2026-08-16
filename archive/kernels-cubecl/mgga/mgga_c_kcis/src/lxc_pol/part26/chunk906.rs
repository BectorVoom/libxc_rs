//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 906/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk906<F: Float>(t1317: F, t21267: F, t1324: F, t11402: F, t6957: F, t1319: F, t5481: F, t5513: F, t3820: F, t6964: F, t11491: F, t5556: F) -> (F, F, F, F, F, F, F) {
    let t21268 = t1317 * t21267;
    let t21270 = t1324 * t21267;
    let t21272 = t11402 * t6957;
    let t21273 = t21272 * t1319;
    let t21275 = t5513 * t5481;
    let t21277 = t3820 * t6964;
    let t21278 = t21277 * t1319;
    let t21280 = t11491 * t6957;
    let t21281 = t21280 * t1319;
    let t21283 = t5556 * t5481;
    (t21268, t21270, t21273, t21275, t21278, t21281, t21283)
}
