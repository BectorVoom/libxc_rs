//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 435/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk435<F: Float>(t124: F, t182: F, t190: F, t625: F, t406: F, t726: F, t58: F, t583: F) -> (F, F, F, F) {
    let t1853 = t124 * t182;
    let t1856 = F::cast_from(0.23744444444444444444e-1_f64) * t625 * t1853 * t190;
    let t1858 = F::cast_from(8.0_f64) * t406 * t726;
    let t1859 = t583 * t58;
    (t1853, t1856, t1858, t1859)
}
