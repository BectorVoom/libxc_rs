//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 314/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk314<F: Float>(t1333: F, t780: F, t787: F, t1325: F, t794: F, t25: F, t1327: F, t785: F, t793: F) -> (F, F, F, F, F) {
    let t1334 = t780 * t1333;
    let t1337 = t787 * t1333;
    let t1339 = t794 * t1325;
    let t1340 = t25 * t1339;
    let t1342 = F::cast_from(0.1898925e1_f64) * t1334 - t785 - F::cast_from(0.29896666666666666667e0_f64) * t1327 + F::cast_from(0.3071625e0_f64) * t1337 - t793 - F::cast_from(0.82156666666666666667e-1_f64) * t1340;
    (t1334, t1337, t1339, t1340, t1342)
}
