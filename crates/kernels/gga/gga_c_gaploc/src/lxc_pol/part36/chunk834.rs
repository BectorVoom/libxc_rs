//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 834/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk834<F: Float>(t1445: F, t41774: F, t597: F, t2787: F, t9127: F, t12894: F, t18658: F, t3085: F, t8097: F, t1457: F, t4540: F, t12918: F, t1562: F, t4614: F) -> (F, F, F, F, F, F, F) {
    let t41777 = F::new(0.11502877786176224903e2) * t597 * t1445 * t41774;
    let t41778 = t2787 * t9127;
    let t41781 = F::new(0.11502877786176224903e2) * t597 * t1445 * t41778;
    let t41783 = F::new(0.21450293971110256001e1) * t18658 * t12894;
    let t41784 = t8097 * t3085;
    let t41787 = F::new(0.21450293971110256001e1) * t4540 * t1457 * t41784;
    let t41790 = F::new(0.92023022289409799224e1) * t1562 * t4614 * t12918;
    (t41777, t41778, t41781, t41783, t41784, t41787, t41790)
}
