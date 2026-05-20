//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3196/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3196<F: Float>(t17191: F, t3566: F, t3781: F, t5216: F, t45618: F, t460: F, t487: F, t43350: F, t44535: F, t45607: F, t13045: F, t1204: F, t17948: F) -> (F, F, F, F, F, F, F) {
    let t59817 = t3566 * t17191;
    let t59854 = t5216 * t3781;
    let t59864 = t460 * t45618 * t487;
    let t59865 = t43350 * t44535;
    let t59871 = t460 * t45607 * t487;
    let t59872 = t43350 * t13045;
    let t59941 = t1204 * t17948;
    (t59817, t59854, t59864, t59865, t59871, t59872, t59941)
}
