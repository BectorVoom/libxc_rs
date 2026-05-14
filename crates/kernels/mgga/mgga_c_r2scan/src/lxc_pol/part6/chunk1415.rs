//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1415/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1415<F: Float>(t22406: F, t21874: F, t21884: F, t21887: F, t21899: F, t21902: F, t22399: F, t22403: F, t22404: F, t22409: F, t22411: F, t22416: F, t22419: F, t2035: F, t2483: F, t41: F) -> (F, F) {
    let t26765 = 144.0 * t22406;
    let t26768 = -t22399 + t22403 + t21874 - t21884 - t21887 + t21899 + 36.0 * t22404 + t21902 - t26765 - t22409 - 0.76213258799999999999e-2 * t22411 - t22416 - 0.20010214504933333333e-1 * t22419;
    let t26770 = t41 * t2483 * t2035;
    (t26768, t26770)
}
