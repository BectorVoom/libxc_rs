//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1334/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1334<F: Float>(t1824: F, t22705: F, t22852: F, t550: F, t59: F, t22827: F, t26297: F, t6943: F, t26301: F, t26322: F, t6936: F, t1831: F, t31176: F) -> (F, F, F, F, F) {
    let t120363 = t22852 * t22705 * t59 * t1824 * t550;
    let t120366 = t22827 * t6943 * t26297;
    let t120369 = t22827 * t6943 * t26301;
    let t120372 = t6936 * t6943 * t26322;
    let t120375 = t31176 * t1831;
    (t120363, t120366, t120369, t120372, t120375)
}
