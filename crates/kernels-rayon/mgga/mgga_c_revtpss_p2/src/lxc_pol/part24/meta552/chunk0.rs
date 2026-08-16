//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1642/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1642(t23467: f64, t52508: f64, t6109: f64, t11385: f64, t2926: f64, t23568: f64, t4719: f64, t23649: f64, t18898: f64, t64043: f64, t981: f64, t1699: f64, t5023: f64, t78478: f64, t88004: f64, t88007: f64, t88012: f64, t88016: f64, t88023: f64, t88026: f64, t88028: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88030 = 0.3859675079686208416e3_f64 * t52508 * t23467;
    let t88031 = t6109 * t6109;
    let t88034 = 0.57895126195293126241e3_f64 * t11385 * t88031 * t2926;
    let t88036 = 0.20779030926817756511e3_f64 * t4719 * t23568;
    let t88038 = 0.4101607543286562663e4_f64 * t4719 * t23649;
    let t88041 = 0.61524113149298439947e4_f64 * t981 * t18898 * t64043;
    let t88042 = -4.0_f64 * t1699 * t5023 * t78478 - t88004 + t88007 - t88012 + t88016 - t88023 + t88026 - t88028 - t88030 + t88034 - t88036 - t88038 - t88041;
    (t88030, t88031, t88034, t88036, t88038, t88041, t88042)
}
