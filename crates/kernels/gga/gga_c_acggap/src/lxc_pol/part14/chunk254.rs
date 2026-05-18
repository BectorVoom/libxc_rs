//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 254/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk254<F: Float>(t1036: F, t1039: F, t330: F, t363: F, t130: F, t328: F, t138: F, t134: F, t342: F) -> (F, F, F, F, F, F) {
    let t1041 = F::new(0.42874018118069736972e-3) * t1036 * t1039;
    let t1044 = t330 * t363;
    let t1046 = t130 * t328;
    let t1047 = t1046 * t138;
    let t1048 = F::new(7.0) / F::new(9.0) * t1047;
    let t1049 = t342 * t134;
    (t1041, t1044, t1046, t1047, t1048, t1049)
}
