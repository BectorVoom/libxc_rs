//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 883/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk883<F: Float>(t323: F, t3242: F, t633: F, t32092: F, t8313: F, t30029: F, t8310: F, t309: F, t8306: F, t32130: F, t7934: F, t32003: F, t322: F, t3919: F, t8347: F, t29991: F, t639: F) -> (F, F, F, F, F, F, F, F) {
    let t33227 = 0.19756347548806534796e1 * t3242 * t633 * t323;
    let t33228 = t32092 * t8313;
    let t33230 = t30029 * t8310;
    let t33232 = t8306 * t309;
    let t33234 = t32130 * t33232 * t7934;
    let t33240 = t32003 * t8306 * t322 * t7934;
    let t33250 = t8347 * t3919;
    let t33256 = t29991 * t639;
    (t33227, t33228, t33230, t33232, t33234, t33240, t33250, t33256)
}
