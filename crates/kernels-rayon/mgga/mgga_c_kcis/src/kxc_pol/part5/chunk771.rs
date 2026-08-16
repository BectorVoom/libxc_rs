//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 771/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk771(t5963: f64, t5996: f64, t552: f64, t573: f64, t5747: f64, t577: f64, t1548: f64, t5906: f64, t5911: f64, t5914: f64, t5917: f64, t5920: f64, t5922: f64, t5924: f64, t5926: f64, t5930: f64, t5933: f64, t5936: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5997 = t5963 + t5996;
    let t5998 = t5997 * t552;
    let t5999 = t5998 * sigma2;
    let t6000 = t5999 * t573;
    let t6002 = t5747 * t577;
    let t6003 = t6002 * t1548;
    let t6005 = -t5906 / 576.0_f64 - t5911 / 72.0_f64 + t5914 / 192.0_f64 + t5917 / 192.0_f64 - t5920 / 24.0_f64 - t5922 / 192.0_f64 + t5924 / 256.0_f64 - t5926 / 16.0_f64 + t5930 / 256.0_f64 - t5933 / 24.0_f64 + t5936 / 36.0_f64 + t6000 / 16.0_f64 + t6003 / 256.0_f64;
    (t5997, t5999, t6000, t6002, t6003, t6005)
}
