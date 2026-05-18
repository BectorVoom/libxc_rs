//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1249/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1249<F: Float>(t37455: F, t40411: F, t42911: F, t42914: F, t42918: F, t42922: F, t42929: F, t42931: F, t42933: F, t42937: F, t42939: F, t42943: F, t42947: F, t42949: F, t42951: F) -> F {
    let t43870 = -t42911 + t42914 - t42918 + t42922 - F::new(0.19211284388664477842e-2) * t37455 + t42929 + t42931 - t42933 - t42937 + t42939 - t42943 - t42947 + t42949 + t42951 - F::new(0.14408463291498358381e-2) * t40411;
    t43870
}
