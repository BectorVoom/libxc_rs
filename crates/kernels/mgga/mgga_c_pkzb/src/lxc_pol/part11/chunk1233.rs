//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1233/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1233<F: Float>(t2754: F, t9164: F, t10806: F, t1861: F, t667: F, t1066: F, t218: F, t219: F, t9161: F, t10767: F, t655: F, t208: F, t29813: F) -> (F, F, F, F, F) {
    let t30328 = t2754 * t9164;
    let t30331 = t1861 * t10806 * t667;
    let t30338 = t218 * t219 * t1066 * t9161;
    let t30342 = t218 * t219 * t655 * t10767;
    let t30346 = t218 * t219 * t208 * t29813;
    (t30328, t30331, t30338, t30342, t30346)
}
