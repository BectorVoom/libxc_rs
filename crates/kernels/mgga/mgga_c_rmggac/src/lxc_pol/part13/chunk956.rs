//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 956/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk956<F: Float>(t39666: F, t7788: F, t262: F, t40833: F, t36254: F, t40805: F, t7782: F, t40808: F, t35929: F, t40738: F, t4669: F, t39688: F, t5271: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40970 = t7788 * t39666;
    let t40972 = t262 * t40833;
    let t40973 = t36254 * t40972;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    let t40978 = t262 * t40808;
    let t40979 = t35929 * t40978;
    let t40981 = t4669 * t40738;
    let t40991 = t5271 * t39688;
    (t40970, t40972, t40973, t40975, t40976, t40978, t40979, t40981, t40991)
}
