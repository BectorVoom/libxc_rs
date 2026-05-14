//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 689/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk689<F: Float>(t5322: F, t6689: F, t5321: F, t1871: F, t2558: F, t1937: F, t196: F, t6884: F, t1646: F, t725: F, t707: F, t1883: F, t1888: F, t1909: F, t2517: F, t2521: F, t2543: F, t5231: F, t7030: F, t7035: F, t7039: F, t7051: F, t7056: F, t7060: F, t709: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7333 = t5322 * t6689;
    let t7334 = t5321 * t7333;
    let t7336 = t2558 * t1871;
    let t7337 = t7336 * sigma2;
    let t7338 = t7337 * t1937;
    let t7340 = t6884 * t196;
    let t7349 = t725 * t1646;
    let t7360 = t725 * t707;
    let t7365 = 0.619125e-2 * t7340 * t709 + 0.9286875e-2 * t2543 * t1883 - 0.619125e-2 * t2543 * t1888 + 0.9286875e-2 * t1909 * t2517 + 0.46434375e-2 * t7349 * t7030 - 0.9286875e-2 * t5231 * t7035 + 0.9286875e-2 * t725 * t7039 - 0.619125e-2 * t1909 * t2521 - 0.9286875e-2 * t5231 * t7051 + 0.123825e-1 * t7360 * t7056 - 0.619125e-2 * t725 * t7060;
    (t7333, t7334, t7336, t7337, t7338, t7340, t7349, t7360, t7365)
}
