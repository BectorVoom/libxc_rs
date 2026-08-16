//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2079/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2079<F: Float>(t26135: F, t3938: F, t12816: F, t191: F, t192: F, t2020: F, t26161: F, t26162: F, t56404: F, t16148: F, t24995: F, t8945: F) -> (F, F, F, F) {
    let t86668 = F::cast_from(27.0_f64) * t3938 * t26135;
    let t86672 = t12816 * t191 * t192;
    let t86673 = t86672 * t2020;
    let t86676 = F::cast_from(4.0_f64) * t26161 * t26162 * t56404;
    let t86679 = F::cast_from(12.0_f64) * t24995 * t8945 * t16148;
    (t86668, t86673, t86676, t86679)
}
