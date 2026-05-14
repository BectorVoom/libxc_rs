//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 692/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk692<F: Float>(t6069: F, t6072: F, t2106: F, t776: F, t162: F, t9: F, t2097: F, t2105: F, t265: F, t254: F, t2146: F, t545: F) -> (F, F, F, F, F, F) {
    let t6073 = t6069 * t6072;
    let t6075 = t776 * t2106;
    let t6077 = t162 * t162;
    let t6079 = 1.0 / t9 / t6077;
    let t6082 = t2097 * t6079 * t265 * t2105;
    let t6084 = 0.1713958891116262235e0 * t254 * t6082;
    let t6085 = t545 * t2146;
    (t6073, t6075, t6077, t6079, t6084, t6085)
}
