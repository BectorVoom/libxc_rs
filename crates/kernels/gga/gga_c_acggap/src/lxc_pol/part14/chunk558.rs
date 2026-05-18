//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 558/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk558<F: Float>(t4282: F, t4284: F, t1470: F, t3409: F, t1410: F, t174: F, t435: F, t1549: F, t1554: F, t1558: F, t1016: F, t524: F) -> (F, F, F, F, F, F, F, F) {
    let t4285 = t4282 * t4284;
    let t4288 = F::new(0.40015750243531754508e-2) * t3409 * t1470;
    let t4289 = t174 * t1410;
    let t4298 = t435 * t1410;
    let t4308 = F::new(0.40015750243531754508e-2) * t3409 * t1549;
    let t4310 = F::new(0.40015750243531754508e-2) * t3409 * t1554;
    let t4312 = F::new(0.20007875121765877254e-2) * t3409 * t1558;
    let t4313 = t1016 * t524;
    (t4285, t4288, t4289, t4298, t4308, t4310, t4312, t4313)
}
