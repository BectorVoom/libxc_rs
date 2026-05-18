//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1210/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1210<F: Float>(t3262: F, t3465: F, t43950: F, t12812: F, t3270: F, t3269: F, t3275: F, t3472: F, t42424: F, t11189: F, t42419: F, t43798: F) -> (F, F, F, F, F) {
    let t44077 = F::new(3.0) / F::new(4.0) * t3262 * t3465 * t43950;
    let t44078 = t3270 * t12812;
    let t44080 = t3269 * t44078 / F::new(4.0);
    let t44083 = F::new(5.0) / F::new(16.0) * t3275 * t3472 * t42424;
    let t44086 = F::new(45.0) / F::new(64.0) * t3275 * t11189 * t42419;
    let t44089 = F::new(5.0) / F::new(8.0) * t3275 * t3472 * t43798;
    (t44077, t44080, t44083, t44086, t44089)
}
