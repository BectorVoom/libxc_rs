//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1249/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1249(t37455: f64, t40411: f64, t42911: f64, t42914: f64, t42918: f64, t42922: f64, t42929: f64, t42931: f64, t42933: f64, t42937: f64, t42939: f64, t42943: f64, t42947: f64, t42949: f64, t42951: f64) -> f64 {
    let t43870 = -t42911 + t42914 - t42918 + t42922 - 0.19211284388664477842e-2_f64 * t37455 + t42929 + t42931 - t42933 - t42937 + t42939 - t42943 - t42947 + t42949 + t42951 - 0.14408463291498358381e-2_f64 * t40411;
    t43870
}
