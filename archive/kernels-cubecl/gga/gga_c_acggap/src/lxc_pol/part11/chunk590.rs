//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 590/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk590<F: Float>(t1165: F, t1188: F, t4298: F, t407: F, t4289: F, t1549: F, t3409: F, t1554: F, t1558: F, t1016: F, t524: F, t1017: F, t157: F) -> (F, F, F, F, F, F, F) {
    let t4300 = t1165 * t4298 * t1188;
    let t4304 = t1165 * t4289 * t407;
    let t4308 = F::cast_from(0.40015750243531754508e-2_f64) * t3409 * t1549;
    let t4310 = F::cast_from(0.40015750243531754508e-2_f64) * t3409 * t1554;
    let t4312 = F::cast_from(0.20007875121765877254e-2_f64) * t3409 * t1558;
    let t4313 = t1016 * t524;
    let t4314 = t157 * t1017;
    (t4300, t4304, t4308, t4310, t4312, t4313, t4314)
}
