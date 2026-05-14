//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1143/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1143<F: Float>(t2722: F, t5867: F, t415: F, t6211: F, t9452: F, t6204: F, t20160: F, t9808: F) -> (F, F, F, F, F) {
    let t33584 = t5867 * t2722;
    let t33585 = t415 * t33584;
    let t33587 = t9452 * t6211;
    let t33588 = t6204 * t33587;
    let t33593 = t20160 * t9808;
    (t33584, t33585, t33587, t33588, t33593)
}
