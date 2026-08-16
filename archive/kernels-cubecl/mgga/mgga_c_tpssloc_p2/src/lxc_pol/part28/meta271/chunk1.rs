//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1158/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1158<F: Float>(t192: F, t533: F, t1390: F, t2094: F, t16: F, t2: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F) -> (F, F, F, F, F, F) {
    let t8944 = t192 * t533;
    let t9016 = t2094 * t1390;
    let t9212 = t2 * t16;
    let t9214 = t9 * t591;
    let t9216 = t587 * t21;
    let t9218 = t14 * t598;
    (t8944, t9016, t9212, t9214, t9216, t9218)
}
