//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1179/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1179<F: Float>(t16615: F, t16617: F, t19754: F, t10666: F, t1769: F, t10647: F, t16416: F, t10634: F, t5381: F, t10630: F, t1727: F, t2639: F, t8914: F) -> (F, F, F, F, F, F, F, F) {
    let t28967 = F::new(0.10389515463408878255e3) * t16615;
    let t28968 = F::new(0.10254018858216406658e4) * t16617;
    let t28970 = F::new(72.0) * t19754;
    let t28977 = t1769 * t10666;
    let t28979 = t16416 * t10647;
    let t28990 = t5381 * t10634;
    let t28992 = t1727 * t10630;
    let t28995 = t8914 * t2639;
    (t28967, t28968, t28970, t28977, t28979, t28990, t28992, t28995)
}
