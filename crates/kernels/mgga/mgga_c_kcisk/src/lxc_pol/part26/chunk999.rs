//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 999/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk999<F: Float>(t19032: F, t25342: F, t3484: F, t5633: F, t26503: F, t5634: F, t3796: F, t1163: F, t8247: F, t3482: F, t3494: F, t7831: F, t1340: F, t1411: F, t3739: F, t7836: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26849 = t19032 * t25342;
    let t26850 = t3484 * t26849;
    let t26851 = t5633 * t26850;
    let t26856 = t5634 * t26503;
    let t26857 = t3796 * t26856;
    let t26858 = t5633 * t26857;
    let t26860 = t8247 * t1163;
    let t26861 = t3796 * t26860;
    let t26862 = t3482 * t26861;
    let t26865 = t3494 * t7831;
    let t26866 = t1340 * t26865;
    let t26867 = t1411 * t26866;
    let t26869 = t3739 * t7836;
    (t26849, t26851, t26856, t26858, t26860, t26862, t26865, t26867, t26869)
}
