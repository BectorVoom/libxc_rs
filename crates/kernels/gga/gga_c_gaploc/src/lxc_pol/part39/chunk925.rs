//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 925/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk925<F: Float>(t41906: F, t34478: F, t544: F, t9287: F, t10318: F, t1397: F, t2487: F, t2754: F, t9438: F, t9448: F, t204: F, t2476: F, t41810: F) -> (F, F, F, F, F) {
    let t41907 = F::cast_from(0.15337170381568299871e1_f64) * t41906;
    let t41909 = t544 * t34478 * t9287;
    let t41914 = t1397 * t10318 * t9287;
    let t41915 = F::cast_from(0.29792074959875355558e-1_f64) * t41914;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    let t41919 = F::cast_from(0.15976219147466979032e-1_f64) * t41918;
    let t41922 = F::cast_from(0.46011511144704899612e1_f64) * t2476 * t204 * t41810;
    (t41907, t41909, t41915, t41919, t41922)
}
