//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 595/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk595<F: Float>(t5963: F, t5996: F, t552: F, t573: F, t5747: F, t577: F, t1548: F, t5906: F, t5911: F, t5914: F, t5917: F, t5920: F, t5922: F, t5924: F, t5926: F, t5930: F, t5933: F, t5936: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t5997 = t5963 + t5996;
    let t5998 = t5997 * t552;
    let t5999 = t5998 * sigma2;
    let t6000 = t5999 * t573;
    let t6002 = t5747 * t577;
    let t6003 = t6002 * t1548;
    let t6005 = -t5906 / 576.0 - t5911 / 72.0 + t5914 / 192.0 + t5917 / 192.0 - t5920 / 24.0 - t5922 / 192.0 + t5924 / 256.0 - t5926 / 16.0 + t5930 / 256.0 - t5933 / 24.0 + t5936 / 36.0 + t6000 / 16.0 + t6003 / 256.0;
    (t5997, t5998, t5999, t6000, t6002, t6003, t6005)
}
