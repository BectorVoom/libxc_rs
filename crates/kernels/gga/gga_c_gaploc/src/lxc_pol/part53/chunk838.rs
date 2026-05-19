//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 838/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk838<F: Float>(t1457: F, t41784: F, t4540: F, t12918: F, t1562: F, t4614: F, t12766: F, t597: F, t40147: F, t3116: F, t986: F) -> (F, F, F, F, F) {
    let t41787 = F::cast_from(0.21450293971110256001e1_f64) * t4540 * t1457 * t41784;
    let t41790 = F::cast_from(0.92023022289409799224e1_f64) * t1562 * t4614 * t12918;
    let t41793 = F::cast_from(0.15337170381568299871e2_f64) * t597 * t4614 * t12766;
    let t41800 = F::cast_from(0.11502877786176224903e1_f64) * t40147;
    let t41809 = t986 * t3116;
    (t41787, t41790, t41793, t41800, t41809)
}
