//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 837/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk837<F: Float>(t12806: F, t4540: F, t4673: F, t3116: F, t7995: F, t1445: F, t597: F, t2787: F, t9127: F, t12894: F, t18658: F, t3085: F, t8097: F) -> (F, F, F, F, F, F, F) {
    let t41773 = F::cast_from(0.14300195980740170667e1_f64) * t4540 * t4673 * t12806;
    let t41774 = t7995 * t3116;
    let t41777 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t41774;
    let t41778 = t2787 * t9127;
    let t41781 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t41778;
    let t41783 = F::cast_from(0.21450293971110256001e1_f64) * t18658 * t12894;
    let t41784 = t8097 * t3085;
    (t41773, t41774, t41777, t41778, t41781, t41783, t41784)
}
