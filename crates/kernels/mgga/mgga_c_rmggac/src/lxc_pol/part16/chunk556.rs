//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 556/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk556<F: Float>(t2227: F, t874: F, t7937: F, t7940: F, t2265: F, t942: F, t2416: F, t7487: F, t2160: F, t2339: F, t638: F, t2323: F, t1540: F, t511: F, t650: F, t1411: F, t2011: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8264 = t874 * t2227;
    let t8303 = 0.1440846329149835838e-2 * t7937;
    let t8304 = 0.39726959900411316772e-4 * t7940;
    let t8310 = t942 * t2265;
    let t8311 = 0.4726e1 * t8310;
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    let t8339 = t1540 * t511;
    let t8340 = t8339 * t650;
    let t8342 = t2011 * t1411;
    (t8264, t8303, t8304, t8311, t8328, t8331, t8334, t8339, t8340, t8342)
}
