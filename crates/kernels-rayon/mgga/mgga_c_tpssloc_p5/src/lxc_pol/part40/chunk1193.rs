//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1193/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1193(t18914: f64, t18939: f64, t475: f64, t1214: f64, t248: f64, t3508: f64, t5011: f64, t4977: f64, t4582: f64, t11692: f64, t1174: f64, t1213: f64, t1227: f64, t15610: f64, t15642: f64, t15645: f64, t18393: f64, t18397: f64, t18401: f64, t18574: f64, t18577: f64, t18580: f64, t18584: f64, t18590: f64, t18594: f64, t3506: f64, t3577: f64, t488: f64, t4974: f64, t4989: f64, t5005: f64, t5024: f64) -> (f64, f64) {
    let t18940 = t18914 + t18939;
    let t18941 = t18940 * t475;
    let t18943 = t248 * t1214 * t18941;
    let t18946 = t3508 * t5011;
    let t18947 = t4977 * t18946;
    let t18948 = t4582 * t18947;
    let t18951 = -t15610 - t18393 / 3456.0_f64 + t11692 * t18397 / 2304.0_f64 - t3577 * t18401 / 1152.0_f64 + t18574 * t488 / 3072.0_f64 + t1174 * t18577 / 108.0_f64 + t1174 * t18580 / 36.0_f64 - t3577 * t18584 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t5005 * t4989 - t1227 * t18590 / 1152.0_f64 - t1227 * t18594 / 768.0_f64 + t5024 * t4974 / 216.0_f64 + t1213 * t18943 / 3072.0_f64 + t15642 - t15645 + t3506 * t18948 / 768.0_f64;
    (t18940, t18951)
}
