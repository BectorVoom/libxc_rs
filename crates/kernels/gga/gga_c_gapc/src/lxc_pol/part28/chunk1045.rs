//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1045/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1045<F: Float>(t12023: F, t12037: F, t209: F, t3804: F, t575: F, t687: F, t1049: F, t10526: F, t10529: F, t2967: F, t3179: F, t3480: F) -> (F, F, F, F, F, F, F, F) {
    let t12038 = t12023 + t12037;
    let t12039 = t12038 * t209;
    let t12040 = t3804 * t575;
    let t12041 = t12040 * t687;
    let t12042 = t10526 * t1049;
    let t12043 = t10529 * t2967;
    let t12044 = F::new(2.0) * t12043;
    let t12045 = t3480 * t3179;
    (t12038, t12039, t12040, t12041, t12042, t12043, t12044, t12045)
}
