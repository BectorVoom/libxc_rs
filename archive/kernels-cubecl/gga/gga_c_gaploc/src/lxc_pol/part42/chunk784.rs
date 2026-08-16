//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 784/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk784<F: Float>(t12957: F, t1441: F, t12939: F, t1407: F, t2754: F, t587: F, t9438: F, t9439: F, t40076: F, t30829: F, t31769: F, t544: F, t913: F) -> (F, F, F, F, F) {
    let t41698 = t1441 * t12957;
    let t41705 = t1407 * t12939;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41736 = F::cast_from(0.25561950635947166451e0_f64) * t40076;
    let t41884 = t544 * t30829 * t913 * t31769;
    (t41698, t41705, t41711, t41736, t41884)
}
