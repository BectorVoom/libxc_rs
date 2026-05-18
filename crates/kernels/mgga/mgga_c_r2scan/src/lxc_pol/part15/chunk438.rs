//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 438/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk438<F: Float>(t1800: F, t650: F, t653: F, t181: F, t648: F, t14: F, t178: F, t651: F) -> (F, F, F, F) {
    let t1888 = F::new(0.16081979498692535067e2) * t650 * t653 * t1800;
    let t1890 = F::new(1.0) / t648 / t181;
    let t1891 = t14 * t1890;
    let t1893 = F::new(1.0) / t651 / t178;
    (t1888, t1890, t1891, t1893)
}
