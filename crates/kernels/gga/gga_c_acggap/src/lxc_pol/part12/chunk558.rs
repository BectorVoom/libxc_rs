//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 558/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk558<F: Float>(t3868: F, t865: F, t1212: F, t310: F, t447: F, t150: F, t443: F, t848: F, t1264: F, t322: F, t449: F, t316: F) -> (F, F, F, F, F, F, F, F) {
    let t3869 = t3868 * t865;
    let t3871 = t310 * t1212;
    let t3873 = t447 * t447;
    let t3874 = F::new(1.0) / t3873;
    let t3875 = t150 * t3874;
    let t3880 = t848 * t443;
    let t3882 = t322 * t1264;
    let t3883 = t449 * t3882;
    let t3884 = t316 * t3883;
    (t3869, t3871, t3873, t3874, t3875, t3880, t3883, t3884)
}
