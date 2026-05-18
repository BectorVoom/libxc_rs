//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 752/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk752<F: Float>(t12361: F, t87: F, t40: F, t7997: F, t10021: F, t8004: F, t4652: F, t4664: F, t4744: F, t4751: F, t4754: F, t4784: F, t4790: F, t4799: F, t4803: F) -> (F, F, F, F, F, F) {
    let t12362 = t12361 * t87;
    let t12363 = t40 * t12362;
    let t12364 = F::new(3.0) * t7997;
    let t12365 = F::new(0.17544670192365612213e1) * t10021;
    let t12366 = F::new(0.73246220147012639764e-3) * t8004;
    let t12367 = t4744 + t4751 + t4652 + t4754 + t12363 + t4664 + t12364 - t12365 - t4784 - t4790 + t12366 - t4799 - t4803;
    (t12362, t12363, t12364, t12365, t12366, t12367)
}
