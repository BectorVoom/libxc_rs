//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 876/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk876<F: Float>(t20984: F, t5662: F, t4170: F, t4160: F, t5627: F, t5632: F, t1468: F, t1464: F, t1889: F, t5676: F, t15887: F, t5880: F) -> (F, F, F, F, F, F, F) {
    let t20985 = t5662 * t20984;
    let t20986 = t4170 * t20985;
    let t20987 = t4160 * t20986;
    let t20989 = t5632 * t5627;
    let t20990 = t1468 * t20989;
    let t20991 = t1464 * t20990;
    let t20994 = t1889 * t5676;
    let t20995 = t15887 * t20994;
    let t20996 = t4160 * t20995;
    let t20998 = t1889 * t5880;
    (t20985, t20987, t20989, t20991, t20994, t20996, t20998)
}
