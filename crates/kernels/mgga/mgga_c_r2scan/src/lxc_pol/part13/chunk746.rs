//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 746/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk746<F: Float>(t162: F, t9: F, t2097: F, t2105: F, t265: F, t254: F, t2146: F, t545: F) -> (F, F, F, F) {
    let t6077 = t162 * t162;
    let t6079 = F::cast_from(1.0_f64) / t9 / t6077;
    let t6082 = t2097 * t6079 * t265 * t2105;
    let t6084 = F::cast_from(0.1713958891116262235e0_f64) * t254 * t6082;
    let t6085 = t545 * t2146;
    (t6077, t6079, t6084, t6085)
}
