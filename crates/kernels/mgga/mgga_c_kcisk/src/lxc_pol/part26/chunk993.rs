//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 993/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk993<F: Float>(t26599: F, t26665: F, t26707: F, t26748: F, t1341: F, t1415: F, t1411: F, t3739: F, t7908: F, t5886: F, t6002: F, t2075: F, t5991: F, t18997: F, t3482: F, t3748: F, t8181: F) -> (F, F, F, F, F, F, F, F) {
    let t26750 = t26599 + t26665 + t26707 + t26748;
    let t26751 = t1341 * t26750;
    let t26752 = t1415 * t26751;
    let t26753 = t1411 * t26752;
    let t26755 = t3739 * t7908;
    let t26757 = t5886 * t6002;
    let t26758 = t1411 * t26757;
    let t26760 = t2075 * t5991;
    let t26761 = t18997 * t26760;
    let t26762 = t3482 * t26761;
    let t26764 = t3748 * t8181;
    (t26750, t26751, t26753, t26755, t26758, t26760, t26762, t26764)
}
