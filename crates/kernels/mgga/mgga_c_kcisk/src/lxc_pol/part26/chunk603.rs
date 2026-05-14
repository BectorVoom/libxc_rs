//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 603/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk603<F: Float>(t2231: F, t3764: F, t1415: F, t1411: F, t2237: F, t3739: F, t2152: F, t3494: F, t1340: F, t1451: F, t5606: F, t2232: F, t3508: F, t1440: F, t1341: F, t3785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5975 = t3764 * t2231;
    let t5976 = t1415 * t5975;
    let t5977 = t1411 * t5976;
    let t5979 = t3739 * t2237;
    let t5981 = t3494 * t2152;
    let t5982 = t1340 * t5981;
    let t5983 = t1411 * t5982;
    let t5985 = t5606 * t1451;
    let t5986 = t1411 * t5985;
    let t5988 = t3508 * t2232;
    let t5989 = t1411 * t5988;
    let t5991 = t2231 * t1440;
    let t5992 = t1341 * t5991;
    let t5993 = t3785 * t5992;
    (t5975, t5976, t5977, t5979, t5981, t5982, t5983, t5985, t5986, t5988, t5989, t5991, t5992, t5993)
}
