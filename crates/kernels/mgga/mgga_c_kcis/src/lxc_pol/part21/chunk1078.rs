//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1078/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1078<F: Float>(t7589: F, t92181: F, t209: F, t2403: F, t2389: F, t2404: F, t705: F, t2410: F, t700: F, t706: F, t7580: F, t26602: F, t26623: F, t26620: F, t92174: F, t91909: F, t91913: F, t91916: F, t91919: F, t91922: F, t92171: F, t92175: F, t92177: F) -> (F, F) {
    let t92182 = t7589 * t92181;
    let t92184 = t209 * t2403;
    let t92187 = t92184 * t2389 * t2404 * t705;
    let t92188 = t7589 * t92187;
    let t92193 = t7589 * t92184 * t706 * t700 * t2410;
    let t92195 = t7580 * t92187;
    let t92197 = t26602 * t26623;
    let t92201 = t26620 * t2389 * t2410 * t705;
    let t92202 = t7589 * t92201;
    let t92204 = t7589 * t92174;
    let t92206 = t7580 * t92181;
    let t92213 = -0.55652820312500000001e-3 * t92171 - 0.2782641015625e-3 * t92175 + 0.97307291666666666666e-2 * t92177 + 0.41703125000000000001e-2 * t92182 + 0.41703125000000000001e-2 * t92188 - 0.41703125000000000001e-2 * t92193 + 0.55652820312500000001e-3 * t92195 - 0.41703125000000000001e-2 * t92197 - 0.208515625e-2 * t92202 - 0.208515625e-2 * t92204 + 0.83479230468750000001e-3 * t92206 - 0.79593333333333333331e-1 * t91909 - 0.59694999999999999999e-1 * t91913 - 0.92858888888888888885e-1 * t91916 - 0.39796666666666666665e-1 * t91919 + 0.39796666666666666665e-1 * t91922;
    (t92201, t92213)
}
