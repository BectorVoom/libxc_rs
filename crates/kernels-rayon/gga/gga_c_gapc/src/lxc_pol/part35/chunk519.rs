//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 519/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk519(t1673: f64, t3021: f64, t1043: f64, t677: f64, t2970: f64, t2976: f64, t2984: f64, t2988: f64, t2991: f64, t3002: f64, t3009: f64, t3014: f64, t3019: f64) -> (f64, f64) {
    let t3022 = t3021 * t1673;
    let t3023 = t1043 * t3022;
    let t3025 = t1043 * t677;
    let t3027 = 0.54106179813099907243e-4_f64 * t2970 + 0.6081664768516204948e-3_f64 * t2976 - 0.10120768229166666667e-4_f64 * t2984 + 0.86880925264517213544e-4_f64 * t2988 + 0.86880925264517213544e-4_f64 * t2991 + 0.84412963981222021454e-7_f64 * t3002 + 0.16882592796244404291e-6_f64 * t3009 - 0.25340269868817520617e-4_f64 * t3014 - 0.72463633678258804342e-6_f64 * t3019 + 0.14492726735651760868e-5_f64 * t3023 - 0.84540905957968605066e-5_f64 * t3025;
    (t3022, t3027)
}
