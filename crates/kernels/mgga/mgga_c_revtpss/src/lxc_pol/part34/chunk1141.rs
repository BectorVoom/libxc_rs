//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1141/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1141<F: Float>(t27254: F, t27256: F, t28034: F, t27924: F, t27926: F, t27929: F, t27937: F, t27955: F, t1450: F, t6816: F, t7237: F, t2014: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28336 = F::new(0.28582678745379824648e-4) * t27254;
    let t28337 = F::new(0.16006300097412701803e-1) * t27256;
    let t28679 = F::new(2.0) / F::new(3.0) * t28034;
    let t28872 = F::new(0.2032800112371413129e-3) * t27924;
    let t28873 = F::new(0.16006300097412701803e-1) * t27926;
    let t28874 = F::new(0.28582678745379824648e-4) * t27929;
    let t28877 = F::new(0.11433071498151929859e-3) * t27937;
    let t28885 = F::new(7.0) / F::new(72.0) * t27955;
    let t29494 = t1450 * t6816;
    let t29495 = t7237 * t29494;
    let t29497 = F::new(3.0) * t2014 * t29495;
    (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t29494, t29495, t29497)
}
