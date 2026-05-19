//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 915/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk915<F: Float>(t12830: F, t29874: F, t39805: F, t39808: F, t39811: F, t12803: F, t1358: F, t12797: F, t12763: F, t6305: F, t2268: F, t2343: F, t41865: F) -> (F, F, F, F, F, F, F, F) {
    let t42820 = t29874 * t12830;
    let t42821 = F::cast_from(0.71137516589190373998e-2_f64) * t42820;
    let t42822 = F::cast_from(0.16598753870811087267e-1_f64) * t39805;
    let t42823 = F::cast_from(0.23712505529730124666e-2_f64) * t39808;
    let t42824 = F::cast_from(0.23712505529730124666e-2_f64) * t39811;
    let t42825 = t1358 * t12803;
    let t42826 = F::cast_from(0.63233348079280332443e-2_f64) * t42825;
    let t42827 = t29874 * t12797;
    let t42828 = F::cast_from(0.23712505529730124666e-2_f64) * t42827;
    let t42829 = t6305 * t12763;
    let t42832 = t2268 * t2343 * t41865;
    (t42821, t42822, t42823, t42824, t42826, t42828, t42829, t42832)
}
