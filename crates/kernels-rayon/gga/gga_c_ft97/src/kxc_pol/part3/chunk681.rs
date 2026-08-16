//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 681/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk681(t3312: f64, t3682: f64, t4026: f64, t4399: f64, t1851: f64, t971: f64, t1882: f64, t3010: f64, t2989: f64, t2994: f64, t2985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10947 = 2.0_f64 * t3312;
    let t10948 = 2.0_f64 * t3682;
    let t10949 = 2.0_f64 * t4026;
    let t10950 = 2.0_f64 * t4399;
    let t10969 = t971 * t1851;
    let t10992 = t1882 * t3010;
    let t10993 = t10992 / 27.0_f64;
    let t11021 = t1882 * t2989;
    let t11022 = t11021 / 27.0_f64;
    let t11023 = t1882 * t2994;
    let t11024 = 2.0_f64 / 27.0_f64 * t11023;
    let t11025 = t1882 * t2985;
    (t10947, t10948, t10949, t10950, t10969, t10992, t10993, t11021, t11022, t11023, t11024, t11025)
}
