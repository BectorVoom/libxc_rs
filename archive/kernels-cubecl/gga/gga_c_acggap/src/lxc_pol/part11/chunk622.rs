//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 622/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk622<F: Float>(t3143: F, t503: F, t1049: F, t1476: F, t1298: F, t301: F) -> (F, F, F, F) {
    let t4814 = t3143 * t503;
    let t4816 = t1049 * t1476;
    let t4817 = F::cast_from(0.1956e1_f64) * t4816;
    let t4818 = t1298 * t301;
    (t4814, t4816, t4817, t4818)
}
