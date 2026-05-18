//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 214/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk214<F: Float>(t621: F, t650: F, t653: F, t230: F, t406: F, t410: F, t229: F, t424: F) -> (F, F, F, F) {
    let t656 = F::new(0.16081979498692535067e2) * t650 * t653 * t621;
    let t658 = F::new(4.0) * t406 * t230;
    let t660 = F::new(4.0) * t410 * t230;
    let t661 = t424 * t229;
    (t656, t658, t660, t661)
}
