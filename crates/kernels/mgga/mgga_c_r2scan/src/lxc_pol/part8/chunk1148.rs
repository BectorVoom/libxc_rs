//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1148/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1148<F: Float>(t1647: F, t1800: F, t188: F, t5672: F, t5674: F, t218: F, t5265: F, t215: F, t5268: F, t212: F, t5332: F, t625: F, t645: F, t5447: F, t5448: F, t5679: F) -> (F, F, F, F, F, F) {
    let t21659 = 0.14973420227703480549e6 * t5672 * t188 * t5674 * t1800 * t1647;
    let t21677 = 1.0 / t5265 / t218;
    let t21680 = 1.0 / t5268 / t215;
    let t21692 = 1.0 / t5265 / t212;
    let t21699 = 0.22161481481481481481e0 * t625 * t5332 * t645;
    let t21702 = 0.24828486201251232144e5 * t5447 * t5679 * t5448;
    (t21659, t21677, t21680, t21692, t21699, t21702)
}
