//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 536/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk536<F: Float>(t2910: F, t1000: F, t1256: F, t2904: F, t308: F, t1001: F, t1268: F, t2901: F, t2905: F, t295: F, t305: F, t309: F, t997: F, tau1: F) -> (F, F, F, F, F, F) {
    let t2911 = tau1 * t2910;
    let t2916 = t1000 * t1000;
    let t2917 = t1256 * t2916;
    let t2920 = -t2904;
    let t2921 = t308 * t2920;
    let t2924 = F::new(10.0) / F::new(9.0) * t295 * t2901 + F::new(5.0) / F::new(3.0) * t295 * t2905 + F::new(40.0) / F::new(9.0) * t2911 * t309 - F::new(50.0) / F::new(9.0) * t997 * t1001 + F::new(10.0) / F::new(9.0) * t305 * t2917 + F::new(5.0) / F::new(3.0) * t305 * t2921 - t1268;
    (t2911, t2916, t2917, t2920, t2921, t2924)
}
