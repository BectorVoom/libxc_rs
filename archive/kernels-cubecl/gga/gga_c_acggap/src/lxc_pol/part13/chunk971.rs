//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 971/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk971<F: Float>(t323: F, t3242: F, t609: F, t7927: F, t872: F, t2130: F, t3874: F, t615: F, t315: F, t7930: F, t309: F, t7932: F, t7934: F) -> (F, F, F, F, F, F) {
    let t32109 = F::cast_from(0.19756347548806534796e1_f64) * t3242 * t609 * t323;
    let t32121 = t7927 * t872;
    let t32123 = t2130 * t3874;
    let t32124 = t615 * t32123;
    let t32130 = t315 * t7930;
    let t32133 = t32130 * t7932 * t309 * t7934;
    (t32109, t32121, t32123, t32124, t32130, t32133)
}
