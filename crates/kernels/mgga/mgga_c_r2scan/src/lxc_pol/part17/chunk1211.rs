//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1211/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1211<F: Float>(t11506: F, t41344: F, t12033: F, t40276: F, t3275: F, t3472: F, t42966: F, t3579: F, t41348: F, t12052: F, t12422: F, t2867: F, t41202: F) -> (F, F, F, F, F, F) {
    let t44091 = F::new(3.0) / F::new(2.0) * t11506 * t41344;
    let t44093 = t40276 * t12033 / F::new(2.0);
    let t44096 = F::new(5.0) / F::new(8.0) * t3275 * t3472 * t42966;
    let t44098 = t3579 * t41348 / F::new(2.0);
    let t44100 = t12422 * t12052 / F::new(4.0);
    let t44103 = t3275 * t41202 * t2867 / F::new(2.0);
    (t44091, t44093, t44096, t44098, t44100, t44103)
}
