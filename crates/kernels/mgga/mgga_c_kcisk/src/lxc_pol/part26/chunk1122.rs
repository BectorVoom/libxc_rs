//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1122/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1122<F: Float>(t32558: F, t32682: F, t233: F, t1065: F, t9406: F, t1152: F, t9789: F, t2752: F, t5586: F, t294: F, t2707: F, t5579: F, t5585: F, t559: F, t2709: F, t9408: F, t9786: F) -> (F, F, F, F, F, F, F, F) {
    let t32683 = t32558 + t32682;
    let t32684 = t233 * t32683;
    let t32685 = t1065 * t9406;
    let t32686 = 2.0 * t32685;
    let t33325 = t1152 * t9789;
    let t33327 = t5586 * t2752;
    let t33328 = t294 * t33327;
    let t33330 = t5579 * t2707;
    let t33331 = t5585 * t559;
    let t33332 = t2709 * t33331;
    let t33334 = t9408 * t9786;
    (t32683, t32684, t32686, t33325, t33328, t33330, t33332, t33334)
}
