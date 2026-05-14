//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1197/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1197<F: Float>(t32558: F, t32682: F, t233: F, t1065: F, t9406: F, t295: F, t3464: F, t2710: F, t2752: F, t3473: F, t294: F, t2707: F, t3299: F, t1152: F, t9789: F, t5586: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32683 = t32558 + t32682;
    let t32684 = t233 * t32683;
    let t32685 = t1065 * t9406;
    let t32687 = t3464 * t295;
    let t32688 = t32687 * t2710;
    let t32689 = t32688 / 8.0;
    let t32690 = t3473 * t2752;
    let t32691 = t294 * t32690;
    let t32692 = t32691 / 16.0;
    let t32693 = t3299 * t2707;
    let t33325 = t1152 * t9789;
    let t33327 = t5586 * t2752;
    (t32683, t32684, t32685, t32687, t32689, t32692, t32693, t33325, t33327)
}
