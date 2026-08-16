//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 200/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk200<F: Float>(t170: F, t607: F, t166: F, t585: F, t159: F, t12: F, t15: F, t2: F) -> (F, F, F, F, F) {
    let t608 = t607 * t170;
    let t611 = t166 * t585;
    let t612 = t159 * t611;
    let t614 = F::cast_from(1.0_f64) / t15 / t12;
    let t615 = t614 * t2;
    (t608, t611, t612, t614, t615)
}
