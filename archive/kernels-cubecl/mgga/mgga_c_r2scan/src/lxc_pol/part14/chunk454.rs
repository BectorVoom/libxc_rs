//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 454/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk454<F: Float>(t166: F, t2055: F, t2056: F, t58: F, t758: F, t423: F, t597: F, t761: F, t1376: F) -> (F, F, F, F, F, F) {
    let t2059 = F::cast_from(0.571528e-1_f64) * t2055 * t166 * t2056;
    let t2060 = t758 * t58;
    let t2061 = t2060 * t423;
    let t2062 = t597 * t761;
    let t2063 = t2061 * t2062;
    let t2065 = t1376 * t166;
    (t2059, t2060, t2061, t2062, t2063, t2065)
}
