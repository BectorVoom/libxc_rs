//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 835/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk835<F: Float>(t12766: F, t4614: F, t597: F, t12905: F, t1641: F, t1445: F, t31501: F, t574: F, t874: F, t40147: F, t12792: F, t158: F) -> (F, F, F, F, F) {
    let t41793 = F::cast_from(0.15337170381568299871e2_f64) * t597 * t4614 * t12766;
    let t41794 = t1641 * t12905;
    let t41798 = t574 * t1445 * t31501 * t874;
    let t41800 = F::cast_from(0.11502877786176224903e1_f64) * t40147;
    let t41801 = t158 * t12792;
    (t41793, t41794, t41798, t41800, t41801)
}
