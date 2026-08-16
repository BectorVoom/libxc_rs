//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 533/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk533<F: Float>(t1421: F, t1459: F, t1511: F, t1513: F, t1526: F, t2810: F, t2813: F, t2816: F, t2866: F, t2869: F, t2872: F, t2879: F, t881: F) -> F {
    let t2881 = t2866 + t1421 - t1511 + t1459 + t2869 - t1526 - F::cast_from(0.2363e1_f64) * t881 * t2816 - F::cast_from(0.2363e1_f64) * t2872 - F::cast_from(0.2363e1_f64) * t881 * t2810 - F::cast_from(0.2363e1_f64) * t881 * t2813 - t1513 + t2879;
    t2881
}
