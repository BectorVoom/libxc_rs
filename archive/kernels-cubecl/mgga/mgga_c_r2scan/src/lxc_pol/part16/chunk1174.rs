//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1174/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1174<F: Float>(t10772: F, t10810: F, t3100: F, t3308: F, t574: F, t9151: F, t8779: F, t12476: F, t37685: F, t10776: F, t9165: F, t9169: F) -> (F, F, F, F, F, F) {
    let t43026 = t10772 * t10810 * t3100;
    let t43029 = t574 * t3308 * t9151;
    let t43032 = t574 * t3308 * t8779;
    let t43034 = t37685 * t12476;
    let t43037 = t10776 * t3308 * t9165;
    let t43040 = t10776 * t3308 * t9169;
    (t43026, t43029, t43032, t43034, t43037, t43040)
}
