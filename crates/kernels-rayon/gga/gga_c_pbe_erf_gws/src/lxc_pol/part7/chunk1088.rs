//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1088/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1088(t18645: f64, t18647: f64, t18655: f64, t18658: f64, t18660: f64, t18662: f64, t18667: f64, t18669: f64, t18701: f64, t18703: f64, t18705: f64, t18707: f64, t19502: f64, t19504: f64, t19505: f64, t2053: f64, t2429: f64, t321: f64, t382: f64, t6837: f64, t944: f64) -> f64 {
    let t19513 = -4.0_f64 * t2053 * t321 * t6837 * t944 + 18.0_f64 * t19505 * t2429 * t382 - t18645 - t18647 + t18655 + t18658 - t18660 + t18662 - t18667 - t18669 + t18701 - t18703 + t18705 + t18707 + t19502 + t19504;
    t19513
}
