//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1075/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1075<F: Float>(t31659: F, t3952: F, t14935: F, t30153: F, t13894: F, t1581: F, t30158: F, t1312: F, t30738: F, t41: F, t30494: F, t6443: F) -> (F, F, F, F, F) {
    let t31660 = t3952 * t31659;
    let t31665 = t14935 * t30153;
    let t31666 = t13894 * t31665;
    let t31669 = t1581 * t30158;
    let t31670 = t1312 * t31669;
    let t31679 = t30738 * t41;
    let t31695 = t6443 * t30494;
    (t31660, t31666, t31670, t31679, t31695)
}
