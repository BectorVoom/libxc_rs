//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 882/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk882<F: Float>(t5963: F, t5996: F, t552: F, t573: F, t5747: F, t577: F, t1548: F, t5906: F, t5911: F, t5914: F, t5917: F, t5920: F, t5922: F, t5924: F, t5926: F, t5930: F, t5933: F, t5936: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t5997 = t5963 + t5996;
    let t5998 = t5997 * t552;
    let t5999 = t5998 * sigma2;
    let t6000 = t5999 * t573;
    let t6002 = t5747 * t577;
    let t6003 = t6002 * t1548;
    let t6005 = -t5906 / F::cast_from(576.0_f64) - t5911 / F::cast_from(72.0_f64) + t5914 / F::cast_from(192.0_f64) + t5917 / F::cast_from(192.0_f64) - t5920 / F::cast_from(24.0_f64) - t5922 / F::cast_from(192.0_f64) + t5924 / F::cast_from(256.0_f64) - t5926 / F::cast_from(16.0_f64) + t5930 / F::cast_from(256.0_f64) - t5933 / F::cast_from(24.0_f64) + t5936 / F::cast_from(36.0_f64) + t6000 / F::cast_from(16.0_f64) + t6003 / F::cast_from(256.0_f64);
    (t5997, t5999, t6000, t6002, t6003, t6005)
}
