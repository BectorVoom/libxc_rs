//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1131/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1131<F: Float>(t10879: F, t11741: F, t37660: F, t39540: F, t39542: F, t39545: F, t39549: F, t39550: F, t39552: F, t39554: F, t39558: F, t39561: F, t39563: F) -> F {
    let t39565 = t10879 * t11741;
    let t39567 = -F::new(0.43663693315433241792e-2) * t39540 - F::new(0.16463622957338778997e0) * t39542 - F::new(0.2600466522016280569e0) * t39545 - F::new(0.14282990759302185292e-1) * t37660 - t39549 - F::new(0.54878743191129263322e-1) * t39550 - F::new(0.43341108700271342816e-1) * t39552 - F::new(0.86682217400542685632e-1) * t39554 - F::new(0.22511059664845582436e0) * t39558 - F::new(0.43341108700271342816e-1) * t39561 - F::new(0.13002332610081402845e0) * t39563 - F::new(0.2600466522016280569e0) * t39565;
    t39567
}
