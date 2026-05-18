//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1349/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1349<F: Float>(t103045: F, t7949: F, t1532: F, t7052: F, t7299: F, t94748: F, t28594: F, t5919: F, t1928: F, t2034: F, t7953: F, t103031: F, t103033: F, t103035: F, t103038: F, t103040: F, t103043: F) -> (F, F, F, F, F, F) {
    let t103046 = t103045 * t7949;
    let t103048 = t1532 * t7052;
    let t103049 = t103048 * t7949;
    let t103051 = t94748 * t7299;
    let t103053 = t28594 * t5919;
    let t103055 = t2034 * t1928;
    let t103056 = t103055 * t7953;
    let t103058 = F::new(0.9375e-1) * t103031 - F::new(0.1875e0) * t103033 - F::new(0.26979166666666666667e-1) * t103035 - F::new(0.9375e-1) * t103038 + F::new(0.5e0) * t103040 - F::new(0.9375e-1) * t103043 + F::new(0.5e0) * t103046 - F::new(0.91666666666666666667e0) * t103049 + F::new(0.53958333333333333334e-1) * t103051 + F::new(0.33333333333333333334e0) * t103053 - F::new(0.33333333333333333333e0) * t103056;
    (t103046, t103049, t103051, t103053, t103056, t103058)
}
