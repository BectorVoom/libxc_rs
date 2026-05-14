//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 515/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk515<F: Float>(t681: F, t689: F, t1691: F, t226: F, t1399: F, t1732: F, t1734: F, t1738: F, t1740: F) -> (F, F, F) {
    let t2017 = t681 * t689;
    let t2021 = t226 * t1691;
    let t2029 = 0.235315e1 * t1732 - 0.62750666666666666667e1 * t1734 - 0.28051666666666666667e0 * t1738 + 0.56103333333333333335e0 * t1740 + 0.13892666666666666667e0 * t1399;
    (t2017, t2021, t2029)
}
