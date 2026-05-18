//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1081/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1081<F: Float>(t10868: F, t2147: F, t6541: F, t6402: F, t10844: F, t10903: F, t2201: F, t10848: F, t2207: F, t10698: F, t10716: F, t10810: F, t1577: F, t6536: F) -> (F, F, F, F, F, F) {
    let t38088 = t2147 * t10868 * t6541;
    let t38093 = t2147 * t10868 * t6402;
    let t38096 = t2201 * t10903 * t10844;
    let t38099 = t2207 * t10903 * t10848;
    let t38111 = t10698 * t10716;
    let t38114 = t1577 * t10810 * t6536;
    (t38088, t38093, t38096, t38099, t38111, t38114)
}
