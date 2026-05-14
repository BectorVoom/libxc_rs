//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 976/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk976<F: Float>(t13306: F, t6235: F, t5622: F, t2214: F, t3805: F, t3739: F, t5969: F, t3494: F, t5996: F, t1415: F, t1411: F, t2231: F, t3502: F, t1450: F, t3732: F, t1341: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19784 = t13306 * t6235;
    let t19786 = t13306 * t5622;
    let t19788 = t3805 * t2214;
    let t19790 = t3739 * t5969;
    let t19791 = 0.33163888888888888888e-2 * t19790;
    let t19792 = t3494 * t5996;
    let t19793 = t1415 * t19792;
    let t19794 = t1411 * t19793;
    let t19796 = t2231 * t3502;
    let t19797 = t1450 * t19796;
    let t19798 = t1415 * t19797;
    let t19799 = t1411 * t19798;
    let t19801 = t2231 * t3732;
    let t19802 = t1341 * t19801;
    (t19784, t19786, t19788, t19790, t19791, t19794, t19796, t19799, t19801, t19802)
}
